# Extractous Python Bindings

This project provides Python bindings for the Extractous library, allowing you to use extractous functionality in
your Python applications.

## Installation

To install the extractous Python bindings, you can use pip:

```bash
pip install extractous
```

## Usage

Extracting a file to string:

```python
from extractous import Extractor

extractor = Extractor()
extractor.set_extract_string_max_length(1000)
result = extractor.extract_file_to_string("README.md")

print(result)
```

Extracting a file(URL / bytearray) to a buffered stream:

```python
from extractous import Extractor

extractor = Extractor()
# for file
reader = extractor.extract_file("tests/quarkus.pdf")
# for url
# reader = extractor.extract_url("https://www.google.com")
# for bytearray
# with open("tests/quarkus.pdf", "rb") as file:
#     buffer = bytearray(file.read())
# reader = extractor.extract_bytes(buffer)

result = ""
buffer = reader.read(4096)
while len(buffer) > 0:
    result += buffer.decode("utf-8")
    buffer = reader.read(4096)

print(result)
```

Extracting a file with OCR:

```python
from extractous import Extractor, TesseractOcrConfig

extractor = Extractor().set_ocr_config(TesseractOcrConfig().set_language("deu"))
result = extractor.extract_file_to_string("../../test_files/documents/eng-ocr.pdf")

print(result)
```
