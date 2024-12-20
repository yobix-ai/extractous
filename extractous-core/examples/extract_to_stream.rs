use extractous::Extractor;
// use std::fs::File; use for bytes
use std::io::{BufReader, Read};

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

    // Extract the provided file content to a string
    let extractor = Extractor::new().set_xml_output(true);
    let (stream, _metadata) = extractor.extract_file(file_path).unwrap();
    // Extract url
    // let stream = extractor.extract_url("https://www.google.com/").unwrap();
    // Extract bytes
    // let mut file = File::open(file_path)?;
    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer)?;
    // let stream= extractor.extract_bytes(&file_bytes).unwrap();

    // Because stream implements std::io::Read trait we can perform buffered reading
    // For example we can use it to create a BufReader
    let mut reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap())
}
