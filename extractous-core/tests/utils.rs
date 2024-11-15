use std::collections::HashMap;

pub fn is_expected_metadata_contained(
    expected: &HashMap<String, Vec<String>>,
    current: &HashMap<String, Vec<String>>,
) -> bool {
    // Check if all keys in `expected` are present in `current` and have identical values
    expected.iter().all(|(key, expected_values)| {
        let actual_values_opt = current.get(key);
        return match actual_values_opt {
            None => {
                println!("expected key = {key} not found !!");
                false
            }
            Some(actual_values) => {
                if actual_values != expected_values {
                    println!(
                        "values for key = {key} differ!! expected = {:?} and actual = {:?}",
                        expected_values, actual_values
                    );
                    false
                } else {
                    true
                }
            }
        };
    })
}
