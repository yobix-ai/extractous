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
extractor.set_extract_string_max_length(1000) # optional, default 500_000
result = extractor.extract_file_to_string("README.md")

print(result)
```

Extracting a file to dict (content, metadata):

```python
from extractous import Extractor

extractor = Extractor()
extractor.set_extract_string_max_length(1000) # optional, default 500_000
ext_dict = extractor.extract_file_to_dict("README.md")

print(ext_dict)
```

Extracting a file to a buffered stream:

```python
from extractous import Extractor

extractor = Extractor()
reader = extractor.extract_file("tests/quarkus.pdf")

result = ""
buffer = reader.read(4096)
while len(buffer) > 0:
    result += buffer.decode("utf-8")
    buffer = reader.read(4096)

print(result)
```