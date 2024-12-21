
<div align="center" style="margin-top: 20px">
    <a href="https://yobix.ai">
    <img height="28px" alt="yobix ai logo" src="https://framerusercontent.com/images/zaqayjWBWNoQmV9MIwSEKf0HBo.png?scale-down-to=512">
    </a>
<h1 style="margin-top: 0; padding-top: 0">Extractous</h1>
</div>

<div align="center">

<a href="https://github.com/yobix-ai/extractous/blob/main/LICENSE">![https://pypi.python.org/pypi/unstructured/](https://img.shields.io/pypi/l/unstructured.svg)</a>
[![](https://img.shields.io/crates/v/extractous)](https://crates.io/crates/extractous)
[![](https://img.shields.io/pypi/v/extractous)](https://pypi.org/project/extractous/)
<img src="https://img.shields.io/github/commit-activity/m/yobix-ai/extractous" alt="Commits per month">
[![Downloads](https://static.pepy.tech/badge/extractous/month)](https://pepy.tech/project/extractous)

</div>

<div align="center">

_Extractous offers a fast and efficient solution for extracting content and metadata from various documents types such as PDF, Word, HTML, and [many other formats](#supported-file-formats).
Our goal is to deliver a fast and efficient comprehensive solution in Rust with bindings for many programming
languages._

</div>

---

**Demo**: showing that [Extractous 🚀](https://github.com/yobix-ai/extractous) is **25x faster** than the popular
[unstructured-io](https://github.com/Unstructured-IO/unstructured) library ($65m in funding and 8.5k GitHub stars).
For complete benchmarking details please consult our [benchmarking repository](https://github.com/yobix-ai/extractous-benchmarks)

![unstructured_vs_extractous](https://github.com/yobix-ai/extractous-benchmarks/raw/main/docs/extractous_vs_unstructured.gif)
<sup>* demo running at 5x recoding speed</sup>

## Why Extractous?

**Extractous** was born out of frustration with the need to rely on external services or APIs for content extraction from unstructured data. Do we really need to call external APIs or run special servers just for content extraction? Couldn't extraction be performed locally and efficiently?

In our search for solutions, **unstructured-io** stood out as the popular and widely-used library for parsing unstructured content with in-process parsing. However, we identified several significant limitations:

- Architecturally, unstructured-io wraps around numerous heavyweight Python libraries, resulting in slow performance and high memory consumption (see our [benchmarks](https://github.com/yobix-ai/extractous-benchmarks) for more details).
- Inefficient in utilizing multiple CPU cores for data processing tasks, which are predominantly CPU-bound. This inefficiency is due to limitations in its dependencies and constraints like the Global Interpreter Lock (GIL), which prevents multiple threads from executing Python bytecode simultaneously.
- As unstructured-io evolves, it is becoming increasingly complicated, transitioning into more of a complex framework and focusing more offering an external API service for text and metadata extraction.

In contrast, **Extractous** maintains a dedicated focus on text and metadata extraction. It achieves significantly faster processing speeds and lower memory utilization through native code execution.

* **Built with Rust:** The core is developed in Rust, leveraging its high performance, memory safety, multi-threading capabilities, and zero-cost abstractions.
* **Extended format support with Apache Tika:** For file formats not natively supported by the Rust core, we compile the well-known [Apache Tika](https://tika.apache.org/) into native shared libraries using [GraalVM](https://www.graalvm.org/) ahead-of-time compilation technology. These shared libraries are then linked to and called from our Rust core. No local servers, no virtual machines, or any garbage collection, just pure native execution.
* **Bindings for many languages:**  we plan to introduce bindings for many languages. At the moment we offer only Python binding, which is essentially is a wrapper around the Rust core with the potential to circumventing the Python GIL limitation and make efficient use of multi-cores.

With Extractous, the need for external services or APIs is eliminated, making data processing pipelines faster and more efficient.

## 🌳 Key Features
* High-performance unstructured data extraction optimized for speed and low memory usage.
* Clear and simple API for extracting text and metadata content.
* Automatically identifies document types and extracts content accordingly
* Supports [many file formats](#supported-file-formats) (most formats supported by Apache Tika).
* Extracts text from images and scanned documents with OCR through [tesseract-ocr](https://github.com/tesseract-ocr/tesseract).
* Core engine written in Rust with bindings for [Python](https://pypi.org/project/extractous/) and upcoming support for JavaScript/TypeScript.
* Detailed documentation and examples to help you get started quickly and efficiently.
* Free for Commercial Use: Apache 2.0 License.

## 🚀 Quickstart
Extractous provides a simple and easy-to-use API for extracting content from various file formats. Below are quick examples:

#### Python
* Extract a file content to a string:
```python
from extractous import Extractor

# Create a new extractor
extractor = Extractor()
extractor = extractor.set_extract_string_max_length(1000)
# if you need an xml
# extractor = extractor.set_xml_output(True)

# Extract text from a file
result, metadata = extractor.extract_file_to_string("README.md")
print(result)
print(metadata)
```
* Extracting a file(URL / bytearray) to a buffered stream:

```python
from extractous import Extractor

extractor = Extractor()
# if you need an xml
# extractor = extractor.set_xml_output(True)

# for file
reader, metadata = extractor.extract_file("tests/quarkus.pdf")
# for url
# reader, metadata = extractor.extract_url("https://www.google.com")
# for bytearray
# with open("tests/quarkus.pdf", "rb") as file:
#     buffer = bytearray(file.read())
# reader, metadata = extractor.extract_bytes(buffer)

result = ""
buffer = reader.read(4096)
while len(buffer) > 0:
    result += buffer.decode("utf-8")
    buffer = reader.read(4096)

print(result)
print(metadata)
```

* Extracting a file with OCR:

You need to have Tesseract installed with the language pack. For example on debian `sudo apt install tesseract-ocr tesseract-ocr-deu`

```python
from extractous import Extractor, TesseractOcrConfig

extractor = Extractor().set_ocr_config(TesseractOcrConfig().set_language("deu"))
result, metadata = extractor.extract_file_to_string("../../test_files/documents/eng-ocr.pdf")

print(result)
print(metadata)
```

#### Rust
* Extract a file content to a string:
```rust
use extractous::Extractor;

fn main() {
    // Create a new extractor. Note it uses a consuming builder pattern
    let mut extractor = Extractor::new().set_extract_string_max_length(1000);
    // if you need an xml
    // extractor = extractor.set_xml_output(true);

    // Extract text from a file
    let (text, metadata) = extractor.extract_file_to_string("README.md").unwrap();
    println!("{}", text);
    println!("{:?}", metadata);
}
```

* Extract a content of a file(URL/ bytes) to a `StreamReader` and perform buffered reading
```rust
use std::io::{BufReader, Read};
// use std::fs::File; use for bytes
use extractous::Extractor;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];

    // Extract the provided file content to a string
    let extractor = Extractor::new();
    // if you need an xml
    // extractor = extractor.set_xml_output(true);

    let (stream, metadata) = extractor.extract_file(file_path).unwrap();
    // Extract url
    // let (stream, metadata) = extractor.extract_url("https://www.google.com/").unwrap();
    // Extract bytes
    // let mut file = File::open(file_path)?;
    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer)?;
    // let (stream, metadata) = extractor.extract_bytes(&file_bytes);

    // Because stream implements std::io::Read trait we can perform buffered reading
    // For example we can use it to create a BufReader
    let mut reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    println!("{}", String::from_utf8(buffer).unwrap());
    println!("{:?}", metadata);
}
```

* Extract content of PDF with OCR.

You need to have Tesseract installed with the language pack. For example on debian `sudo apt install tesseract-ocr tesseract-ocr-deu`

```rust
use extractous::Extractor;

fn main() {
  let file_path = "../test_files/documents/deu-ocr.pdf";

    let extractor = Extractor::new()
          .set_ocr_config(TesseractOcrConfig::new().set_language("deu"))
          .set_pdf_config(PdfParserConfig::new().set_ocr_strategy(PdfOcrStrategy::OCR_ONLY));
    // extract file with extractor
  let (content, metadata) = extractor.extract_file_to_string(file_path).unwrap();
  println!("{}", content);
  println!("{:?}", metadata);
}
```


## 🔥 Performance
* **Extractous** is fast, please don't take our word for it, you can run the [benchmarks](https://github.com/yobix-ai/extractous-benchmarks) yourself. For example extracting content out of [sec10 filings pdf forms](https://github.com/yobix-ai/extractous-benchmarks/raw/main/dataset/sec10-filings), Extractous is on average **~18x faster** than unstructured-io:

![extractous_speedup_relative_to_unstructured](https://github.com/yobix-ai/extractous-benchmarks/raw/main/docs/extractous_speedup_relative_to_unstructured.png)

* Not just speed it is also memory efficient, Extractous allocates **~11x less memory** than unstructured-io:

![extractous_memory_efficiency_relative_to_unstructured](https://github.com/yobix-ai/extractous-benchmarks/raw/main/docs/extractous_memory_efficiency_relative_to_unstructured.png)

* You might be questioning the quality of the extracted content, gues what we even do better in that regard:

![extractous_memory_efficiency_relative_to_unstructured](https://github.com/yobix-ai/extractous-benchmarks/raw/main/docs/extractous_unstructured_quality_scores.png)

## 📄 Supported file formats

| **Category**        | **Supported Formats**                                   | **Notes**                                      |
|---------------------|---------------------------------------------------------|------------------------------------------------|
| **Microsoft Office**| DOC, DOCX, PPT, PPTX, XLS, XLSX, RTF                    | Includes legacy and modern Office file formats |
| **OpenOffice**      | ODT, ODS, ODP                                           | OpenDocument formats                           |
| **PDF**             | PDF                                                     | Can extracts embedded content and supports OCR |
| **Spreadsheets**    | CSV, TSV                                                | Plain text spreadsheet formats                 |
| **Web Documents**   | HTML, XML                                               | Parses and extracts content from web documents |
| **E-Books**         | EPUB                                                    | EPUB format for electronic books               |
| **Text Files**      | TXT, Markdown                                           | Plain text formats                             |
| **Images**          | PNG, JPEG, TIFF, BMP, GIF, ICO, PSD, SVG                | Extracts embedded text with OCR                |
| **E-Mail**          | EML, MSG, MBOX, PST                                     | Extracts content, headers, and attachments     |

[//]: # (| **Archives**        | ZIP, TAR, GZIP, RAR, 7Z                                 | Extracts content from compressed archives      |)
[//]: # (| **Audio**           | MP3, WAV, OGG, FLAC, AU, MIDI, AIFF, APE                | Extracts metadata such as ID3 tags             |)
[//]: # (| **Video**           | MP4, AVI, MOV, WMV, FLV, MKV, WebM                      | Extracts metadata and basic information        |)
[//]: # (| **CAD Files**       | DXF, DWG                                                | Supports CAD formats for engineering drawings  |)
[//]: # (| **Other**           | ICS &#40;Calendar&#41;, VCF &#40;vCard&#41;                             | Supports calendar and contact file formats     |)
[//]: # (| **Geospatial**      | KML, KMZ, GeoJSON                                       | Extracts geospatial data and metadata          |)
[//]: # (| **Font Files**      | TTF, OTF                                                | Extracts metadata from font files              |)

## 🤝 Contributing
Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or new features to propose.

## 🕮 License
This project is licensed under the Apache License 2.0. See the LICENSE file for details.
