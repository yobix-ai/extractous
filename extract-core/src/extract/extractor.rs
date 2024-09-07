use strum_macros::{Display, EnumString};
use crate::errors::ExtractResult;
use crate::{Reader};
use crate::extract::tika;

#[derive(Debug)]
pub struct PdfParserConfig {
    pub extract_inline_images: bool,
    pub extract_marked_content: bool,
}

impl Default for PdfParserConfig {
    fn default() -> Self {
        Self {
            extract_inline_images: true,
            extract_marked_content: false,
        }
    }
}

impl PdfParserConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_extract_inline_images(&mut self, val: bool) -> &mut Self {
        self.extract_inline_images = val;
        self
    }
    pub fn set_extract_marked_content(&mut self, val: bool) -> &mut Self {
        self.extract_marked_content = val;
        self
    }
}

#[derive(Debug, Default)]
pub struct OfficeParserConfig {
    concatenate_phonetic_runs: bool,
    extract_all_alternatives_from_msg: bool,
    extract_macros: bool,
    include_deleted_content: bool,
    include_headers_and_footers: bool,
    include_missing_rows: bool,
}

#[derive(Debug, Default)]
pub struct TesseractOcrConfig {
    apply_rotation: bool,
    density: i32,
    depth: i32,
    enable_image_preprocessing: bool,
    language: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[allow(non_camel_case_types)]
pub enum CharSet {
    #[default]
    UTF_8,
    US_ASCII,
    UTF_16BE
}

pub struct Extractor {
    encoding: CharSet,
    pdf_config: PdfParserConfig,
    office_config: OfficeParserConfig,
    ocr_config: TesseractOcrConfig
}

impl Extractor {

    pub fn new() -> Self {
        Self {
            encoding: CharSet::default(),
            pdf_config: PdfParserConfig::default(),
            office_config: OfficeParserConfig::default(),
            ocr_config: TesseractOcrConfig::default()
        }
    }

    pub fn set_encoding(&mut self, encoding: CharSet) -> &mut Self {
        self.encoding = encoding;
        self
    }
    pub fn set_pdf_config(&mut self, config: PdfParserConfig) -> &mut Self {
        self.pdf_config = config;
        self
    }
    pub fn set_office_config(&mut self, config: OfficeParserConfig) -> &mut Self {
        self.office_config = config;
        self
    }
    pub fn set_ocr_config(&mut self, config: TesseractOcrConfig) -> &mut Self {
        self.ocr_config = config;
        self
    }


    pub fn extract_file<'a>(&'a self, file_path: &'a str) -> ExtractResult<Reader> {
        tika::parse_file(file_path, &self.pdf_config)
    }
    pub fn extract_file_to_string<'a>(&'a self, file_path: &'a str, max_length: i32) -> ExtractResult<String> {
        tika::parse_file_to_string(file_path, max_length)
    }

}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use crate::Extractor;

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
        let result = extractor.extract_file_to_string(TEST_FILE, 10000);
        let content = result.unwrap();
        assert_eq!(content.trim(), expected_content.trim());
    }
}