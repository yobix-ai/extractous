use criterion::{criterion_group, criterion_main, Criterion};

use extractous::Extractor;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

fn extract_to_stream(c: &mut Criterion) {
    let file_path = "../test_files/documents/2022_Q3_AAPL.pdf";
    let extractor = Extractor::new();

    c.bench_function("extract_to_stream", |b| {
        b.iter(|| {
            // Extract the provided file content to a stream
            let stream = extractor.extract_file(file_path).unwrap();
            // Because stream implements std::io::Read trait we can perform buffered reading
            // For example we can use it to create a BufReader
            let mut reader = BufReader::new(stream);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();
        })
    });
}

fn extract_to_string(c: &mut Criterion) {
    let file_path = "../test_files/documents/2022_Q3_AAPL.pdf";
    let extractor = Extractor::new();

    c.bench_function("extract_to_string", |b| {
        b.iter(|| {
            // Extract the provided file content to a string
            let _content = extractor.extract_file_to_string(file_path).unwrap();
        })
    });
}

// Saves results to test_files/criterion which is added to git unlike the default target/criterion
criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(30)
        .without_plots()
        .output_directory(&PathBuf::from_str("../test_files/criterion").unwrap());
    targets = extract_to_stream, extract_to_string
}
criterion_main!(benches);
