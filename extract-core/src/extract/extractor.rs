use crate::errors::ExtractResult;
use crate::extract::{OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};
use crate::tika;
use strum_macros::{Display, EnumString};

/// Supported encodings
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[allow(non_camel_case_types)]
pub enum CharSet {
    #[default]
    UTF_8,
    US_ASCII,
    UTF_16BE,
}

/// Extractor for extracting text from different file formats
///
/// The Extractor uses the builder pattern to set configurations. This allows configuring and
/// extracting text in one line. For example
/// ```rust
/// use extract_rs::{CharSet, Extractor};
/// let text = Extractor::new()
///             .set_extract_string_max_length(1000)
///             .extract_file_to_string("README.md");
/// println!("{}", text.unwrap());
/// ```
///
pub struct Extractor {
    extract_string_max_length: i32,
    encoding: CharSet,
    pdf_config: PdfParserConfig,
    office_config: OfficeParserConfig,
    ocr_config: TesseractOcrConfig,
}

impl Default for Extractor {
    fn default() -> Self {
        Self {
            extract_string_max_length: 10000,
            encoding: CharSet::UTF_8,
            pdf_config: PdfParserConfig::default(),
            office_config: OfficeParserConfig::default(),
            ocr_config: TesseractOcrConfig::default(),
        }
    }
}

impl Extractor {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn new_from(extractor: &Extractor) -> Self {
    //     Self {
    //         extract_string_max_length: extractor.extract_string_max_length,
    //         encoding: extractor.encoding,
    //         pdf_config: extractor.pdf_config.clone(),
    //         office_config: extractor.office_config.clone(),
    //         ocr_config: extractor.ocr_config.clone(),
    //     }
    // }

    /// Set the maximum length of the extracted text. Used only for extract_to_string functions
    /// Default: 10000
    pub fn set_extract_string_max_length(mut self, max_length: i32) -> Self {
        self.extract_string_max_length = max_length;
        self
    }

    /// Set the encoding to use for when extracting text to a stream.
    /// Not used for extract_to_string functions.
    /// Default: CharSet::UTF_8
    pub fn set_encoding(mut self, encoding: CharSet) -> Self {
        self.encoding = encoding;
        self
    }

    /// Set the configuration for the PDF parser
    pub fn set_pdf_config(mut self, config: PdfParserConfig) -> Self {
        self.pdf_config = config;
        self
    }

    /// Set the configuration for the Office parser
    pub fn set_office_config(mut self, config: OfficeParserConfig) -> Self {
        self.office_config = config;
        self
    }
    /// Set the configuration for the Tesseract OCR
    pub fn set_ocr_config(mut self, config: TesseractOcrConfig) -> Self {
        self.ocr_config = config;
        self
    }

    /// Extracts text from a file path. Returns a stream of the extracted text
    /// the stream is decoded using the extractor's `encoding`
    pub fn extract_file<'a>(
        &'a self,
        file_path: &'a str,
    ) -> ExtractResult<impl std::io::Read + 'a> {
        tika::parse_file(file_path, &self.encoding,
                         &self.pdf_config, &self.office_config, &self.ocr_config,
        )
    }

    /// Extracts text from a file path. Returns a string that is of maximum length
    /// of the extractor's `extract_string_max_length`
    pub fn extract_file_to_string<'a>(&'a self, file_path: &'a str) -> ExtractResult<String> {
        tika::parse_file_to_string(file_path, self.extract_string_max_length)
    }
}

#[cfg(test)]
mod tests {
    use crate::Extractor;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    const TEST_FILE: &str = "README.md";

    fn expected_content() -> String {
        let mut file = File::open(TEST_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn extract_file_test() {
        // Prepare expected_content
        let expected_content = expected_content();

        // Parse the files using extract_rs
        let extractor = Extractor::new();
        let result = extractor.extract_file(TEST_FILE);
        let mut reader = BufReader::new(result.unwrap());
        let mut content = String::new();
        reader.read_to_string(&mut content).unwrap();

        assert_eq!(content.trim(), expected_content.trim());

        // let mut reader = BufReader::new(result.unwrap());
        // let mut line = String::new();
        // let _len = reader.read_line(&mut line).unwrap();
        //assert_eq!("# Extract-RS", line.trim());
    }

    #[test]
    fn extract_file_to_string_test() {
        // Prepare expected_content
        let expected_content = expected_content();

        // Parse the files using extract_rs
        let extractor = Extractor::new();
        let result = extractor.extract_file_to_string(TEST_FILE);
        let content = result.unwrap();
        assert_eq!(content.trim(), expected_content.trim());
    }
}