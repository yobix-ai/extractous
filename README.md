
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

**Extractous** was born out of frustration with requiring yet another service to handle content extraction out of 
unstructured data. Do we really need to call external APIs or run special servers just for content extraction? Can't 
we perform the extraction locally and efficiently?

While researching this space, **unstructured-io** offers a good solution for parsing unstructured content, and can be 
performed in-process. However, it's performance is very poor and has many limitations:
* **unstructured-io** wraps around so many heavy Python libraries making it both slow and memory hungry [See benchmarks foo more details](https://github.com/yobix-ai/extractous-benchmarks).
* data processing is mainly a cpu-bound problem and Python is not the best choice for such tasks
  because of its Global Interpreter Lock (GIL) which makes it hard to utilize multiple cores.
* **unstructured-io** is becoming increasingly complex as it focuses on becoming more of a framework rather than 
  just a text and metadata extraction library.

In contrast, **Extractous** is built in Rust, a language renowned for its memory safety and high performance. By leveraging Rust's multithreading capabilities and zero-cost abstractions, Extractous achieves significantly faster processing speeds. **Extractous** maintains a dedicated focus on text and metadata extraction, ensuring optimized performance and reliability in its core functionality.

## 🌳 Key Features
* Fast and efficient unstructured data extraction.
* Clear and simple API for extracting text and metadata content.
* Autodetect document type and extracts content accordingly.
* Supports [many file formats](#supported-file-formats).
* Extracts text from images and scanned documents with OCR through [tesseract-ocr](https://github.com/tesseract-ocr/tesseract).
* Leverages Rust performance and memory safety and provides bindings for [Python](https://pypi.org/project/extractous/) 
  and Javascript/Typescript(coming soon)
* Comprehensive documentation and examples to help you get started quickly.
* Free for Commercial Use: Apache 2.0 License.

## 🚀 Quickstart
Extractous provides a simple and easy-to-use API for extracting content from various file formats. Below are quick examples:

#### Python
* Extract a file content to a string:
```python
from extractous import Extractor

# Create a new extractor
extractor = Extractor()
extractor.set_extract_string_max_length(1000)

# Extract text from a file
result = extractor.extract_file_to_string("README.md")
print(result)
```

#### Rust
* Extract a file content to a string:
```rust
use extractous::Extractor;
use extractous::PdfParserConfig;

// Create a new extractor. Note it uses a consuming builder pattern
let mut extractor = Extractor::new().set_extract_string_max_length(1000);

// Extract text from a file
let text = extractor.extract_file_to_string("README.md").unwrap();
println!("{}", text);
```

## 🔥 Performance
* **Extractous** is fast, please don't take our word for it, you can run the [benchmarks](https://github.com/yobix-ai/extractous-benchmarks) yourself. For example extracting content out of [sec10 filings pdf forms](https://github.com/yobix-ai/extractous-benchmarks/raw/main/dataset/sec10-filings), Extractous is **22x faster** than unstructured-io:

![extractous_speedup_relative_to_unstructured](https://github.com/yobix-ai/extractous-benchmarks/raw/main/docs/extractous_speedup_relative_to_unstructured.png)

* Not just speed it is also memory efficient, Extractous allocates **12x less memory** than unstructured-io:

![extractous_memory_efficiency_relative_to_unstructured](https://github.com/yobix-ai/extractous-benchmarks/raw/main/docs/extractous_memory_efficiency_relative_to_unstructured.png)



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