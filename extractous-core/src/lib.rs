//! Extractous is a library that extracts text from various file formats.
//! * Supports many file formats such as Word, Excel, PowerPoint, PDF, and many more.
//! * Strives to be simple fast and efficient
//!
//! # Quick Start
//! Extractous API entry point is the [`Extractor`] struct.
//! All public apis are accessible through an extractor.
//! The extractor provides functions to extract text and metadata from files, Urls, and byte arrays.
//! To use an extractor, you need to:
//! - [create and configure new the extractor](#create-and-config-an-extractor)
//! - [use the extractor to extract text](#extract-text)
//!
//! ## Create and config an extractor
//!
//! ```no_run
//! use extractous::Extractor;
//! use extractous::PdfParserConfig;
//!
//! // Create a new extractor. Note it uses a consuming builder pattern
//! let mut extractor = Extractor::new()
//!                       .set_extract_string_max_length(1000);
//!
//! // can also perform conditional configuration
//! let custom_pdf_config = true;
//! if custom_pdf_config {
//!     extractor = extractor.set_pdf_config(
//!         PdfParserConfig::new().set_extract_annotation_text(false)
//!     );
//! }
//!
//! ```
//!
//! ## Extract text
//!
//! ```no_run
//! use extractous::Extractor;
//! use extractous::PdfParserConfig;
//!
//! // Create a new extractor. Note it uses a consuming builder pattern
//! let mut extractor = Extractor::new().set_extract_string_max_length(1000);
//!
//! // Extract text and metadata from a file
//! let ext = extractor.extract_file_to_struct("README.md").unwrap();
//! println!("{}", ext.content);
//! println!("{:?}", ext.metadata);
//!
//! ```


/// Default buffer size
pub const DEFAULT_BUF_SIZE: usize = 32768;

// errors module
mod errors;
pub use errors::*;

// extractor module is the config interface
mod config;
pub use config::*;
// extractor module is the main public api interface
mod extractor;
pub use extractor::*;

// tika module, not exposed outside this crate
mod tika {
    mod jni_utils;
    mod parse;
    mod wrappers;
    pub use parse::*;
    pub use wrappers::JReaderInputStream;
    pub use wrappers::JResult;
}
