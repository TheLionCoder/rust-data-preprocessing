use crate::context::salary_model::SalaryRecord;
use csv::ReaderBuilder;

pub(crate) fn load_csv_data(
    csv_data: &str,
) -> Result<Vec<SalaryRecord>, Box<dyn std::error::Error>> {
    let mut records: Vec<_> = Vec::new();
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());
    for result in reader.deserialize() {
        let record: SalaryRecord = result?;
        records.push(record);
    }
    Ok(records)
}
