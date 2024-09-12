use extractous::Extractor;
use std::io::{BufReader, Read};

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

    // Extract the provided file content to a string
    let extractor = Extractor::new();
    let stream = extractor.extract_file(file_path).unwrap();
    // Because stream implements std::io::Read trait we can perform buffered reading
    // For example we can use it to create a BufReader
    let mut reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap())
}
