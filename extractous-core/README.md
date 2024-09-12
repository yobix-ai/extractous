# Extractous

Extractous is a Rust crate that provides a unified approach for detecting and extracting metadata and text content from
various documents
types such as PDF, Word, HTML, and [many other formats](#supported-file-formats).

## Features

* High-level Rust API for extracting text and metadata content for [many file formats](#supported-file-formats).
* Strives to be efficient and fast.
* Internally it calls the [Apache Tika](https://tika.apache.org/) for any file format that is not natively supported in the Rust core.
* Comprehensive documentation and examples to help you get started quickly.

## Installation

To use extractous in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
extractous = "0.1.3"
```

## Usage

* Create and configure an `Extractor` instance
```rust
use extractous::Extractor;
use extractous::PdfParserConfig;

fn main() {
    // Create a new extractor. Note it uses the consuming builder pattern
    let mut extractor = Extractor::new()
        .set_extract_string_max_length(1000);

    // can also perform conditional configuration
    let custom_pdf_config = true;
    if custom_pdf_config {
        extractor = extractor.set_pdf_config(
            PdfParserConfig::new().set_extract_annotation_text(false)
        );
    }
}
```

* Extracting a content of a file to a `String`
```rust
use extractous::Extractor;

fn main() {
  // Get the command-line arguments
  let args: Vec<String> = std::env::args().collect();
  let file_path = &args[1];

  // Extract the provided file content to a string
  let extractor = Extractor::new();
  let content = extractor.extract_file_to_string(file_path).unwrap();
  println!("{}", content);
}
```

* Extract a content of a file to a `StreamReader` and perform buffered reading
```rust
use std::io::{BufReader, Read};
use extractous::Extractor;

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
```

## Building

### Requirements
* Extractous uses [Apache Tika](https://tika.apache.org/) for file formats that are not natively supported in Rust. 
  However, to achieve one of Extractous goals, which is speed and efficiency, we do not set up any Tika as a servers or 
  run any Java code. We instead, compile [Apache Tika](https://tika.apache.org/) as native shared libraries and use 
  them on our Rust core as ffi. [GraalVm](https://www.graalvm.org/) is required to build Tika as native libs. 
* The provided build script already takes care of installing the required GraalVM JDK. However, if you want to use a 
  specific local version, you can do so by setting the GRAALVM_HOME environment variable
* We recommend using [sdkman](https://sdkman.io/install) to install GraalVM JDKs
* `sdk install java 22.0.1-graalce`
* Confirm that GraalVM is installed correctly by running `java -version`. You should see something like:
```text
openjdk 22.0.1 2024-04-16
OpenJDK Runtime Environment Liberica-NIK-24.0.1-1 (build 22.0.1+10)
OpenJDK 64-Bit Server VM Liberica-NIK-24.0.1-1 (build 22.0.1+10, mixed mode, sharing)
```
* On macOS the official GraalVM JDKs fail to work with code that use java awt. On macOS, we recommend using
  Bellsoft Liberica NIK
* `sdk install java 24.0.1.r22-nik`

### Building Extractous
* To build Extractous, just run: 
* `cargo build`

### Running Tests
* To run tests, just run:
* `cargo test`