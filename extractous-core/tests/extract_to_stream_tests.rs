extern crate test_case;
extern crate textdistance;

use std::collections::HashMap;
use extractous::{Extractor, PdfOcrStrategy, PdfParserConfig, TesseractOcrConfig};
use std::fs;
use std::io::Read;
use test_case::test_case;
use textdistance::nstr::cosine;

mod utils;

#[test_case("2022_Q3_AAPL.pdf", 0.9; "Test PDF file")]
#[test_case("science-exploration-1p.pptx", 0.9; "Test PPTX file")]
#[test_case("simple.odt", 0.8; "Test ODT file")]
#[test_case("table-multi-row-column-cells-actual.csv", 0.8; "Test CSV file")]
#[test_case("vodafone.xlsx", 0.4; "Test XLSX file")]
#[test_case("category-level.docx", 0.9; "Test DOCX file")]
#[test_case("simple.doc", 0.9; "Test DOC file")]
#[test_case("simple.pptx", 0.9; "Test another PPTX file")]
#[test_case("table-multi-row-column-cells.png", -1.0; "Test PNG file")]
#[test_case("winter-sports.epub", 0.9; "Test EPUB file")]
#[test_case("bug_16.docx", 0.9; "Test bug16 DOCX file")]
//#[test_case("eng-ocr.pdf", 0.9; "Test eng-ocr PDF file")]
fn test_extract_bytes_to_stream(file_name: &str, target_dist: f64) {
    let extractor = Extractor::new();

    let bytes = fs::read(&format!("../test_files/documents/{}", file_name)).unwrap();
    let mut stream = extractor.extract_bytes(&bytes).unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    let extracted = String::from_utf8_lossy(&buffer);

    // read expected string
    let expected =
        fs::read_to_string(format!("../test_files/expected_result/{}.txt", file_name)).unwrap();

    let dist = cosine(&expected, &extracted);
    assert!(
        dist > target_dist,
        "Cosine similarity is less than {} for file: {}, dist: {}",
        target_dist,
        file_name,
        dist
    );
    println!("{}: {}", file_name, dist);
}

#[test_case("2022_Q3_AAPL.pdf", 0.9; "Test PDF file")]
#[test_case("science-exploration-1p.pptx", 0.9; "Test PPTX file")]
#[test_case("simple.odt", 0.9; "Test ODT file")]
#[test_case("table-multi-row-column-cells-actual.csv", 0.6; "Test CSV file")]
#[test_case("vodafone.xlsx", 0.8; "Test XLSX file")]
#[test_case("category-level.docx", 0.9; "Test DOCX file")]
#[test_case("simple.doc", 0.9; "Test DOC file")]
#[test_case("simple.pptx", 0.9; "Test another PPTX file")]
#[test_case("table-multi-row-column-cells.png", 0.9; "Test PNG file")]
#[test_case("winter-sports.epub", 0.8; "Test EPUB file")]
#[test_case("bug_16.docx", 0.9; "Test bug16 DOCX file")]
//#[test_case("eng-ocr.pdf", 0.8; "Test eng-ocr PDF file")]
fn test_extract_bytes_to_stream_with_metadata(file_name: &str, expected_similarity: f64) {
    /*
    Note: Expected_similarity exists because the extracted metadata may vary across different platforms, but most of it should still match
     */
    let extractor = Extractor::new();

    let bytes = fs::read(&format!("../test_files/documents/{}", file_name)).unwrap();
    let (_expected_stream_readewr, extracted_metadata) = extractor.extract_bytes_with_metadata(&bytes).unwrap();
    let expected_metadata_string =
        fs::read_to_string(format!("../test_files/expected_result/{}.metadata.json", file_name)).unwrap();
    let expected_metadata: HashMap<String, Vec<String>> = serde_json::from_str(&expected_metadata_string).expect("JSON was not well-formatted");
    let percent_similarity = utils::calculate_similarity_percent(&expected_metadata, &extracted_metadata);
    assert!(
        percent_similarity > expected_similarity,
        "The metadata similarity is lower than expected. Current {}% | filename: {}",
        percent_similarity,
        file_name
    );

}

#[test]
fn test_extract_bytes_to_stream_ara_ocr_png() {
    let extractor = Extractor::new()
        .set_ocr_config(TesseractOcrConfig::new().set_language("ara"))
        .set_pdf_config(PdfParserConfig::new().set_ocr_strategy(PdfOcrStrategy::NO_OCR));

    // extract file with extractor
    let bytes = fs::read(&"../test_files/documents/ara-ocr.png".to_string()).unwrap();
    let mut stream = extractor.extract_bytes(&bytes).unwrap();

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    let extracted = String::from_utf8_lossy(&buffer);

    println!("{}", extracted);

    // read expected string
    let expected =
        fs::read_to_string("../test_files/expected_result/ara-ocr.png.txt".to_string()).unwrap();

    let dist = cosine(&expected, &extracted);
    assert!(
        dist > 0.9,
        "Cosine similarity is less than 0.9 for file: ara-ocr.png, dist: {}",
        dist
    );
    println!("{}: {}", "ara-ocr.png", dist);
}
