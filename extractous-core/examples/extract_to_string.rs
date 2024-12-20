use extractous::Extractor;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

    // Extract the provided file content to a string
    let extractor = Extractor::new().set_xml_output(true);
    let (content, _metadata) = extractor.extract_file_to_string(file_path).unwrap();
    println!("{}", content);
}
