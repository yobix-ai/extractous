# Extract-RS

Extract-RS is a Rust crate that provides a unified approach for detecting and extracting metadata and text content from
various documents
types such as PDF, Word, HTML, and [many other formats](#supported-file-formats).

## Features

* High-level Rust API for extracting text and metadata content for [many file formats](#supported-file-formats).
* Strives to be efficient and fast.
* Internally it calls the [Apache Tika](https://tika.apache.org/) for any file format that is not natively supported in the Rust core.
* Comprehensive documentation and examples to help you get started quickly.

## Installation

To use extract-rs in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
extract-rs = "0.1.0"
```

## Supported file formats

| File Format | Native Rust | Through Tika | 
|-------------|-------------|--------------| 
| pdf         | -           | ✅            |
| csv         | ✅           | -            |