
use crate::errors::{ExtractResult, Error};

pub fn extract(
    file_name: &str,
) -> ExtractResult<String> {

    Err(Error::ParseError(format!("extract {} Not implemented yet", file_name)))?
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