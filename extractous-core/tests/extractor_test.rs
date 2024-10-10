extern crate test_case;
extern crate textdistance;
use std::fs::File;
use std::io::{self, BufRead};
use extractous::Extractor;
use std::fs;
use test_case::test_case;
use textdistance::nstr::cosine;

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
fn test_extract_file_to_string(file_name: &str, target_dist: f64) {
    let extractor = Extractor::new().set_extract_string_max_length(1000000);
    // extract file with extractor
    let mut extracted = extractor
        .extract_file_to_struct(&format!("../test_files/documents/{}", file_name))
        .unwrap();
    extracted.metadata.sort();
    // read expected string
    let expected =
        fs::read_to_string(format!("../test_files/expected_result/{}.txt", file_name)).unwrap();
    let expected_metadata = read_metadata_vector(file_name).unwrap();
    let dist = cosine(&expected, &extracted.content);
    assert!(
        dist > target_dist,
        "Cosine similarity is less than {} for file: {}, dist: {}",
        target_dist,
        file_name,
        dist
    );
    assert!(
        extracted.metadata.len() > 0,
        "Metadata should contain at least one entry"
    );
    assert_eq!(extracted.metadata, expected_metadata);
    println!("{}: {}", file_name, dist);
}


fn read_metadata_vector(file_name: &str) -> io::Result<Vec<String>> {
    let file = File::open(format!("../test_files/expected_result/{}.metadata.txt", file_name))?;
    let reader = io::BufReader::new(file);
    let mut expected_metadata = Vec::new();
    for line in reader.lines() {
        expected_metadata.push(line?);
    }
    Ok(expected_metadata)
}
