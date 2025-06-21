use reqwest::blocking::get;

pub(crate) fn fetch_dataset(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(get(url)?.text()?)
}
