use crate::{
    data_loading::load_csv_data,
    encoding::data_encoding::one_hot_encode,
    feature_engineering::feature_enginnering::company_size_score,
    fetch_dataset::fetch_dataset,
    scaling_data::scaling_data::{calculate_mean, calculate_std_dev},
};
use rand::prelude::SliceRandom;
use std::collections::{HashMap, HashSet};

mod context;
mod data_loading;
mod encoding;
mod feature_engineering;
mod fetch_dataset;
mod scaling_data;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";
    let mut experience_level_mapping = HashMap::new();
    let mut rng = rand::rng();

    let csv_data = fetch_dataset(url)?;
    let dataset = load_csv_data(&csv_data)?;

    // View sample records
    for record in dataset.iter().take(5) {
        println!("{:?}", record)
    }

    // Unique job titles
    let unique_job_titles: HashSet<_> = dataset.iter().map(|r| &r.job_title).collect();
    println!("Number of unique job titles: {}", unique_job_titles.len());

    // Average Salary
    let average_salary =
        dataset.iter().map(|r| r.salary_in_usd).sum::<f64>() / dataset.len() as f64;
    println!("Average Salary in USD {:.2}", average_salary);

    // Missing values
    let missing_salaries = dataset.iter().map(|r| r.salary_in_usd.is_nan()).count();
    println!(
        "Number of records with missing salary values: {}",
        missing_salaries
    );

    // Remove duplicates
    let mut seen = HashSet::new();
    let mut unique_dataset = Vec::new();
    for record in dataset {
        let key = (
            record.work_year,
            record.job_title.clone(),
            record.company_location.clone(),
        );
        if seen.insert(key) {
            unique_dataset.push(record);
        }
    }
    println!(
        "Dataset size after removing duplicates: {}",
        unique_dataset.len()
    );

    // Standarize text
    for record in &mut unique_dataset {
        record.experience_level = record.experience_level.to_lowercase();
    }
    let experience_levels: HashSet<_> =
        unique_dataset.iter().map(|r| &r.experience_level).collect();
    println!("Experience levels: {:?}", experience_levels);

    // Scaling numerical data
    let salaries: Vec<f64> = unique_dataset.iter().map(|r| r.salary_in_usd).collect();
    let mean_salary = calculate_mean(&salaries);
    let std_dev_salary = calculate_std_dev(&salaries, mean_salary);

    let standarized_salaries: Vec<f64> = salaries
        .iter()
        .map(|&s| (s - mean_salary) / std_dev_salary)
        .collect();
    println!(
        "Standarized Salaries(first 5): {:?}",
        &standarized_salaries[..5]
    );

    // Encoding categorical data
    let job_titles: Vec<String> = unique_dataset.iter().map(|r| r.job_title.clone()).collect();
    let (enconded_job_titles, job_title_mapping) = one_hot_encode(&job_titles);
    println!(
        "One Hot job titles(first record): {:?}",
        &enconded_job_titles[0]
    );

    // Label encoding
    let experience_levels: Vec<String> = unique_dataset
        .iter()
        .map(|r| r.experience_level.clone())
        .collect();
    let unique_experience_levels: Vec<String> = experience_levels
        .iter()
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    for (i, level) in unique_experience_levels.iter().enumerate() {
        experience_level_mapping.insert(level.clone(), i);
    }
    let encoded_experience_levels: Vec<usize> = experience_levels
        .iter()
        .map(|level| experience_level_mapping[level])
        .collect();
    println!(
        "Encoded experience levels(first 5): {:?}",
        &encoded_experience_levels[..5]
    );

    // Feature engineering
    let remote_work_indicator: Vec<u8> = unique_dataset
        .iter()
        .map(|r| if r.remote_ratio == 100 { 1 } else { 0 })
        .collect();
    println!(
        "Remote work indicator(first 5): {:?}",
        &remote_work_indicator[..5]
    );

    let company_size_scores: Vec<u8> = unique_dataset
        .iter()
        .map(|r| company_size_score(&r.company_size))
        .collect();
    println!(
        "Company size scores(first 5): {:?}",
        &company_size_scores[..5]
    );

    // Data splitting
    let mut idxs: Vec<usize> = (0..unique_dataset.len()).collect();
    idxs.shuffle(&mut rng);
    let train_size = (0.8 * unique_dataset.len() as f64) as usize;
    let train_idxs = &idxs[..train_size];
    let test_idxs = &idxs[train_size..];

    let train_data: Vec<_> = train_idxs.iter().map(|&i| &unique_dataset[i]).collect();
    let test_data: Vec<_> = test_idxs.iter().map(|&i| &unique_dataset[i]).collect();
    println!("Training set size: {}", train_data.len());
    println!("Testing set size: {}", test_data.len());
    Ok(())
}
