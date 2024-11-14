use std::collections::HashMap;

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