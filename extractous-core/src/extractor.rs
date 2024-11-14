use crate::errors::ExtractResult;
use crate::tika;
use crate::tika::Metadata;
use crate::tika::JReaderInputStream;
use crate::{OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};
use strum_macros::{Display, EnumString};

/// CharSet enum of all supported encodings
#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[allow(non_camel_case_types)]
pub enum CharSet {
    #[default]
    UTF_8,
    US_ASCII,
    UTF_16BE,
}
/*
/// StreamReader implements std::io::Read
///
/// Can be used to perform buffered reading. For example:
/// ```rust
/// use extractous::{CharSet, Extractor};
/// use std::io::BufReader;
/// use std::io::prelude::*;
///
/// let extractor = Extractor::new();
/// let reader = extractor.extract_file("README.md").unwrap();
///
/// let mut buf_reader = BufReader::new(reader);
/// let mut content = String::new();
/// buf_reader.read_to_string(&mut content).unwrap();
/// println!("{}", content);
/// ```
///

 */
pub struct StreamReader {
    pub(crate) inner: JReaderInputStream,
}

impl std::io::Read for StreamReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
/*
/// Extractor for extracting text from different file formats
///
/// The Extractor uses the builder pattern to set configurations. This allows configuring and
/// extracting text in one line. For example
/// ```rust
/// use extractous::{CharSet, Extractor};
/// let text = Extractor::new()
///             .set_extract_string_max_length(1000)
///             .extract_file_to_string("README.md");
/// println!("{}", text.unwrap());
/// ```
///

 */
#[derive(Debug, Clone)]
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
            extract_string_max_length: 500_000, // 500KB
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

    /// Set the maximum length of the extracted text. Used only for extract_to_string functions
    /// Default: 500_000
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
    pub fn extract_file(&self, file_path: &str) -> ExtractResult<StreamReader> {
        tika::parse_file(
            file_path,
            &self.encoding,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        ).map(|(stream_reader, _metadata)| stream_reader)
    }

    /// Extracts text from a file path. Returns a tuple with stream of the extracted text and metadata.
    /// the stream is decoded using the extractor's `encoding`
    pub fn extract_file_with_metadata(&self, file_path: &str) -> ExtractResult<(StreamReader, Metadata)> {
        tika::parse_file(
            file_path,
            &self.encoding,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        )
    }

    /// Extracts text from a byte buffer. Returns a stream of the extracted text
    /// the stream is decoded using the extractor's `encoding`
    pub fn extract_bytes(&self, buffer: &[u8]) -> ExtractResult<StreamReader> {
        tika::parse_bytes(
            buffer,
            &self.encoding,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        ).map(|(stream_reader, _metadata)| stream_reader)
    }

    /// Extracts text from a byte buffer. Returns a tuple with stream of the extracted text and metadata.
    /// the stream is decoded using the extractor's `encoding`
    pub fn extract_bytes_with_metadata(&self, buffer: &[u8]) -> ExtractResult<(StreamReader, Metadata)> {
        tika::parse_bytes(
            buffer,
            &self.encoding,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        )
    }

    /// Extracts text from an url. Returns a stream of the extracted text
    /// the stream is decoded using the extractor's `encoding`
    pub fn extract_url(&self, url: &str) -> ExtractResult<StreamReader> {
        tika::parse_url(
            url,
            &self.encoding,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        ).map(|(stream_reader, _metadata)| stream_reader)
    }

    /// Extracts text from an url. Returns a tuple with stream of the extracted text and metadata.
    /// the stream is decoded using the extractor's `encoding`
    pub fn extract_url_with_metadata(&self, url: &str) -> ExtractResult<(StreamReader, Metadata)> {
        tika::parse_url(
            url,
            &self.encoding,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        )
    }

    /// Extracts text from a file path. Returns a string that is of maximum length
    /// of the extractor's `extract_string_max_length`
    pub fn extract_file_to_string(&self, file_path: &str) -> ExtractResult<String> {
        tika::parse_file_to_string(
            file_path,
            self.extract_string_max_length,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        ).map(|(content, _metadata)| content)
    }

    /// Extracts text from a file path. Returns a tuple with string that is of maximum length
    /// of the extractor's `extract_string_max_length` and metadata.
    pub fn extract_file_to_string_with_metadata(&self, file_path: &str) -> ExtractResult<(String, Metadata)> {
        tika::parse_file_to_string(
            file_path,
            self.extract_string_max_length,
            &self.pdf_config,
            &self.office_config,
            &self.ocr_config,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Extractor;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::{self, Read};
    use super::StreamReader;

    const TEST_FILE: &str = "README.md";

    const TEST_URL: &str = "https://www.google.com/";


    fn expected_content() -> String {
        let mut file = File::open(TEST_FILE).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    }

    #[test]
    fn extract_file_to_string_test() {
        // Prepare expected_content
        let expected_content = expected_content();

        // Parse the files using extractous
        let extractor = Extractor::new();
        let result = extractor.extract_file_to_string(TEST_FILE);
        let content = result.unwrap();
        assert_eq!(content.trim(), expected_content.trim());
    }

    #[test]
    fn extract_file_to_string_with_metadata_test() {
        // Parse the files using extractous
        let extractor = Extractor::new();
        let result = extractor.extract_file_to_string_with_metadata(TEST_FILE);
        let (_content, metadata) = result.unwrap();
        assert!(
            metadata.len() > 0,
            "Metadata should contain at least one entry"
        );
    }

    fn read_content_from_stream(stream: StreamReader) -> String {
        let mut reader = BufReader::new(stream);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let content = String::from_utf8(buffer).unwrap();
        content
    }

    #[test]
    fn extract_file_test() {
        // Prepare expected_content
        let expected_content = expected_content();

        // Parse the files using extractous
        let extractor = Extractor::new();
        let result = extractor.extract_file(TEST_FILE);
        let content = read_content_from_stream(result.unwrap());
        assert_eq!(content.trim(), expected_content.trim());
    }

    #[test]
    fn extract_file_with_metadata_test() {
        // Parse the files using extractous
        let extractor = Extractor::new();
        let result = extractor.extract_file_with_metadata(TEST_FILE);
        let (_content, metadata) = result.unwrap();
        assert!(
            metadata.len() > 0,
            "Metadata should contain at least one entry"
        );
    }

    fn read_file_as_bytes(path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn extract_bytes_test() {
        // Prepare expected_content
        let expected_content = expected_content();

        // Parse the bytes using extractous
        let file_bytes = read_file_as_bytes(TEST_FILE).unwrap();
        let extractor = Extractor::new();
        let result = extractor.extract_bytes(&file_bytes);
        let content = read_content_from_stream(result.unwrap());
        assert_eq!(content.trim(), expected_content.trim());
    }

    #[test]
    fn extract_bytes_with_metadata_test() {
        // Parse the bytes using extractous
        let file_bytes = read_file_as_bytes(TEST_FILE).unwrap();
        let extractor = Extractor::new();
        let result = extractor.extract_bytes_with_metadata(&file_bytes);
        let (_content, metadata) = result.unwrap();
        assert!(
            metadata.len() > 0,
            "Metadata should contain at least one entry"
        );
    }

    #[test]
    fn extract_url_test() {
        // Parse url by extractous
        let extractor = Extractor::new();
        let result = extractor.extract_url(&TEST_URL);
        let content = read_content_from_stream(result.unwrap());
        assert!(content.contains("Google"));
    }

    #[test]
    fn extract_url_with_metadata_test() {
        // Parse url by extractous
        let extractor = Extractor::new();
        let result = extractor.extract_url_with_metadata(&TEST_URL);
        let (_content, metadata) = result.unwrap();
        assert!(
            metadata.len() > 0,
            "Metadata should contain at least one entry"
        );
    }
}
