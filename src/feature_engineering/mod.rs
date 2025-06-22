pub mod feature_enginnering {
    pub fn company_size_score(size: &str) -> u8 {
        match size {
            "S" => 1,
            "M" => 2,
            "L" => 3,
            _ => 0,
        }
    }
}
