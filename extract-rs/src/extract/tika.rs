
use crate::errors::{Error, ExtractResult};

pub fn parse_to_string(
    file_name: &str,
) -> ExtractResult<String> {

    Err(Error::ParseError(format!("parse_to_string {} Not implemented yet", file_name)))?
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_to_string_test() {
        let res = parse_to_string("README.md");
        assert!(res.is_err());
    }
}