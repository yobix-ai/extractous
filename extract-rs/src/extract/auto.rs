use crate::errors::ExtractResult;
use crate::tika::{tika_parse_file};

pub fn extract(
    file_name: &str,
) -> ExtractResult<String> {

    tika_parse_file(file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_test() {
        let res = extract("README.md");
        assert!(res.is_err());
    }
}