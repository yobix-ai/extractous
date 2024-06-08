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

## Building

* GraalVm and Gradle are required to build tika_native. We recommend using [sdkman](https://sdkman.io/install)
* To be able to use awt on macOS, please use Bellsoft Liberica NIK java 22
* `sdk install java 24.0.1.r22-nik`
* `sdk install gradle 8.8`
* Make sure gradle is using the graalvm jdk and not any other jdk in your environment. Try `gradle --version` you should see something like:

```text
------------------------------------------------------------
Gradle 8.8
------------------------------------------------------------
Build time:   2024-05-31 21:46:56 UTC
Revision:     4bd1b3d3fc3f31db5a26eecb416a165b8cc36082

Kotlin:       1.9.22
Groovy:       3.0.21
Ant:          Apache Ant(TM) version 1.10.13 compiled on January 4 2023
JVM:          22.0.1 (BellSoft 22.0.1+10)
OS:           Mac OS X 13.2 x86_64
```
