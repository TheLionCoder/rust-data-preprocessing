pub mod data_encoding {
    pub fn one_hot_encode(
        values: &[String],
    ) -> (Vec<Vec<u8>>, std::collections::HashMap<String, usize>) {
        let mut value_to_index = std::collections::HashMap::new();
        let unique_values: Vec<String> = values
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        for (i, value) in unique_values.iter().enumerate() {
            value_to_index.insert(value.clone(), i);
        }
        let encoded_values = values
            .iter()
            .map(|value| {
                let mut vector = vec![0u8; unique_values.len()];
                if let Some(&idx) = value_to_index.get(value) {
                    vector[idx] = 1;
                }
                vector
            })
            .collect();
        (encoded_values, value_to_index)
    }
}
