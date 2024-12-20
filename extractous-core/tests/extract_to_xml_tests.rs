use extractous::Extractor;
use std::fs;
use test_case::test_case;
use textdistance::nstr::cosine;
use quick_xml::reader::Reader;
use quick_xml::events::Event;


// Declarers the shared test_utils code as module in this integration test
mod test_utils;

fn extract_p_tag_content(xml: &str) -> String {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true); // Trim surrounding whitespace
    let mut buf = Vec::new();
    let mut collected_content = String::new();
    let mut inside_body = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"body" => {
                inside_body = true;
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"body" => {
                inside_body = false;
            }
            Ok(Event::Text(e)) if inside_body => {
                collected_content.push_str(&e.unescape().unwrap().into_owned());
                collected_content.push('\n'); // Separate paragraphs with newline
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error reading XML: {}", e);
                break;
            }
            _ => (),
        }
        buf.clear();
    }

    collected_content.trim_end().to_string()
}

#[test_case("2022_Q3_AAPL.pdf", 0.9; "Test PDF file")]
#[test_case("science-exploration-1p.pptx", 0.9; "Test PPTX file")]
#[test_case("simple.odt", 0.8; "Test ODT file")]
#[test_case("table-multi-row-column-cells-actual.csv", 0.8; "Test CSV file")]
#[test_case("vodafone.xlsx", 0.4; "Test XLSX file")]
#[test_case("category-level.docx", 0.8; "Test DOCX file")]
#[test_case("simple.doc", 0.8; "Test DOC file")]
#[test_case("simple.pptx", 0.9; "Test another PPTX file")]
#[test_case("table-multi-row-column-cells.png", -1.0; "Test PNG file")]
#[test_case("winter-sports.epub", 0.8; "Test EPUB file")]
#[test_case("bug_16.docx", 0.9; "Test bug16 DOCX file")]
//#[test_case("eng-ocr.pdf", 0.9; "Test eng-ocr PDF file")]
fn test_extract_file_to_xml(file_name: &str, target_dist: f64) {
    let extractor = Extractor::new().set_extract_string_max_length(1000000)
        .set_xml_output(true);
    // extract file with extractor
    let (extracted_xml, extracted_metadata) = extractor
        .extract_file_to_string(&format!("../test_files/documents/{}", file_name))
        .unwrap();
    println!("{}: {}", file_name, extracted_xml);
    let extracted = extract_p_tag_content(&extracted_xml);

    // read expected string
    let expected =
        fs::read_to_string(format!("../test_files/expected_result/{}.txt", file_name)).unwrap();

    let dist = cosine(&expected.trim(), &extracted.trim());
    assert!(
        dist > target_dist,
        "Cosine similarity is less than {} for file: {}, dist: {}",
        target_dist,
        file_name,
        dist
    );
    println!("{}: {}", file_name, dist);

    // read expected metadata
    let expected_metadata = test_utils::parse_metadata_file(&format!(
        "../test_files/expected_result/{}.metadata.json",
        file_name
    ));

    assert!(test_utils::is_expected_metadata_contained(
        &expected_metadata,
        &extracted_metadata
    ));
}
