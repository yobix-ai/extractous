
<div align="center" style="font-size: xx-large; font-weight: bold">
    <img height="28px" alt="yobix ai logo" src="https://framerusercontent.com/images/zaqayjWBWNoQmV9MIwSEKf0HBo.png?
scale-down-to=512"> Extractous
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
Our goal is to deliver an efficient comprehensive solution with bindings for many programming languages._

</div>

---

**Demo**: showing that [Extractous 🚀](https://github.com/yobix-ai/extractous) is **25x faster** than the popular
[unstructured-io](https://github.com/Unstructured-IO/unstructured) library ($65m in funding and 8.5k GitHub stars). 
For complete benchmarking details please consult our [benchmarking repository](https://github.com/yobix-ai/extractous-benchmarks)

![unstructured_vs_extractous](https://github.com/yobix-ai/extractous-benchmarks/raw/main/docs/extractous_vs_unstructured.gif)
<sup>* demo running at 5x recoding speed </sup>

## Why Extractous?

Extractous was mainly inspired by the [Unstructured Python library](https://github.com/Unstructured-IO/unstructured).
While Unstructured offers a good solution for parsing unstructured content, we see 2 main issues with it:

* Performance: data processing is mainly a cpu-bound problem and Python is not the best choice for such tasks
  because of its Global Interpreter Lock (GIL) which makes it hard to utilize multiple cores.
* [Unstructured](https://github.com/Unstructured-IO/unstructured) is becoming more of an LLM framework rather than
  just text and metadata parsing library.

Extractous will focus only on the text and metadata extraction part. The core is written in Rust, leveraging its
memory safety, multithreading and zero cost abstractions. Extractous will provide bindings for many programming
languages.

## Features

* Clear simple API for extracting text and metadata content.
* Support for [many file formats](#supported-file-formats).
* Strives to be efficient and fast.
* Comprehensive documentation and examples to help you get started quickly.

## Supported file formats

| File Format | Rust Core | Python Binding |
|-------------|-----------|----------------|
| pdf         | ✅         | ✅              |
| csv         | ✅         | ✅              |