# extract-rs Python Bindings

This project provides Python bindings for the extract-rs library, allowing you to use extract-rs functionality in your Python applications.

## Installation

To install the extract-rs Python bindings, you can use pip:

```bash
pip install extract-rs
```

## Usage

extract pdf example:

```python
import extractrs

elements = extractrs.extract_pdf("/tmp/test.txt")
print(elements)
```
