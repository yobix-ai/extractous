use crate::ecore;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::io::Read;

// PyO3 supports unit-only enums (which contain only unit variants)
// These simple enums behave similarly to Python's enumerations (enum.Enum)
#[pyclass(eq, eq_int)]
#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CharSet {
    UTF_8,
    US_ASCII,
    UTF_16BE,
}

impl From<CharSet> for ecore::CharSet {
    fn from(charset: CharSet) -> Self {
        match charset {
            CharSet::UTF_8 => ecore::CharSet::UTF_8,
            CharSet::US_ASCII => ecore::CharSet::US_ASCII,
            CharSet::UTF_16BE => ecore::CharSet::UTF_16BE,
        }
    }
}

#[pyclass]
pub struct StreamReader {
    pub(crate) reader: ecore::StreamReader,
    pub(crate) buffer: Vec<u8>,
}

#[pymethods]
impl StreamReader {

    /// Reads the requested number of bytes
    /// Returns the bytes read as a `bytes` object
    pub fn read<'py>(&mut self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyBytes>> {
        // Resize the buffer to the requested size
        self.buffer.resize(size, 0);

        match self.reader.read(&mut self.buffer) {
            Ok(bytes_read) => {
                // Truncate buffer to actual read size.
                self.buffer.truncate(bytes_read);

                let py_bytes = PyBytes::new_bound(py, &self.buffer);
                Ok(py_bytes)
            }
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "{}",
                e
            ))),
        }
    }
}

/// `Extractor` is the entry for all extract APIs
///
/// Create a new `Extractor` with the default configuration.
#[pyclass]
pub struct Extractor(ecore::Extractor);

#[pymethods]
impl Extractor {
    #[new]
    pub fn new() -> Self {
        Extractor(ecore::Extractor::new())
    }

    pub fn set_extract_string_max_length(&self, max_length: i32) -> Self {
        let inner = self.0.clone().set_extract_string_max_length(max_length);
        Self(inner)
    }

    pub fn set_encoding(&self, encoding: CharSet) -> PyResult<Self> {
        let inner = self.0.clone().set_encoding(encoding.into());
        Ok(Self(inner))
    }

    // pub fn set_pdf_config(&self, config: ecore::PdfParserConfig) -> PyResult<Self> {
    //     let inner = self.0.clone().set_pdf_config(config);
    //     Ok(Self(inner))
    // }

    pub fn extract_file_to_string(&self, filename: &str) -> PyResult<String> {
        self.0
            .extract_file_to_string(filename)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))
    }

    pub fn extract_file(&self, filename: &str) -> PyResult<StreamReader> {
        let reader = self
            .0
            .extract_file(filename)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        // Create a new `StreamReader` with initial buffer capacity of 4096 bytes
        Ok(StreamReader {
            reader,
            buffer: Vec::with_capacity(4096),
        })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}