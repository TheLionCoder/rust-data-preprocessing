use crate::{
    data_loading::load_csv_data,
    fetch_dataset::fetch_dataset,
    scaling_data::scaling_data::{calculate_mean, calculate_std_dev},
};
use std::collections::HashSet;

mod context;
mod data_loading;
mod fetch_dataset;
mod scaling_data;

#[allow(unused_variables)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url =
        "https://raw.githubusercontent.com/kittenpub/database-repository/main/ds_salaries.csv";

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

    Ok(())
}
