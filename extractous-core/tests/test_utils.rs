use std::collections::HashMap;

#[allow(dead_code)]
pub fn parse_metadata_file(file_path: &str) -> HashMap<String, Vec<String>> {
    let expected_metadata_string = std::fs::read_to_string(file_path).unwrap();

    serde_json::from_str(&expected_metadata_string).expect("JSON was not well-formatted")
}

#[allow(dead_code)]
pub fn calculate_similarity_percent(
    expected: &HashMap<String, Vec<String>>,
    current: &HashMap<String, Vec<String>>,
) -> f64 {
    let mut matches = 0;
    let mut total = 0;

    // Iterate over all keys in the 'expected' HashMap
    for (key, value1) in expected {
        if let Some(value2) = current.get(key) {
            total += 1;
            if value1 == value2 {
                matches += 1;
            }
        }
    }
    if total == 0 {
        return 0.0;
    }
    // Return the similarity percentage
    (matches as f64) / (total as f64)
}

#[allow(dead_code)]
pub fn is_expected_metadata_contained(
    expected: &HashMap<String, Vec<String>>,
    current: &HashMap<String, Vec<String>>,
) -> bool {
    // Check if all keys in `expected` are present in `current` and have identical values
    expected.iter().all(|(key, expected_values)| {
        let actual_values_opt = current.get(key);
        match actual_values_opt {
            None => {
                println!("\nexpected key = {key} not found !!");
                false
            }
            Some(actual_values) => {
                if actual_values != expected_values {
                    println!(
                        "\nvalues for key = {key} differ!! expected = {:?} and actual = {:?}",
                        expected_values, actual_values
                    );
                    false
                } else {
                    true
                }
            }
        }
    })
}
