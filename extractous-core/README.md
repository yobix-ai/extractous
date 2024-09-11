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
extractous = "0.1.1"
```

## Supported file formats

| File Format | Native Rust | Through Tika | 
|-------------|-------------|--------------| 
| pdf         | -           | ✅            |
| csv         | ✅           | -            |

## Building

* GraalVm is required to build tika_native. We recommend using [sdkman](https://sdkman.io/install)
* To be able to use awt on macOS, please use Bellsoft Liberica NIK java 22
* `sdk install java 24.0.1.r22-nik`
* We use gradle to perform the build. Gradle wrapper is included in the project, no need to install gradle.
* Make sure `JAVA_HOME` is pointing to the graalvm jdk and not any other jdk in your environment. Try `java --version` 
  you should see something like:

```text
openjdk 22.0.1 2024-04-16
OpenJDK Runtime Environment Liberica-NIK-24.0.1-1 (build 22.0.1+10)
OpenJDK 64-Bit Server VM Liberica-NIK-24.0.1-1 (build 22.0.1+10, mixed mode, sharing)
```