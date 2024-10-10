use crate::{ecore, OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyByteArray, PyList};
use std::io::Read;

// PyO3 supports unit-only enums (which contain only unit variants)
// These simple enums behave similarly to Python's enumerations (enum.Enum)
/// CharSet enum of all supported encodings
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

/// StreamReader represents a stream of bytes
///
/// Can be used to perform buffered reading.
#[pyclass]
pub struct StreamReader {
    pub(crate) reader: ecore::StreamReader,
    pub(crate) buffer: Vec<u8>,
    pub(crate) py_bytes: Option<Py<PyByteArray>>,
}

#[pymethods]
impl StreamReader {
    /// Reads the requested number of bytes
    /// Returns the bytes read as a `bytes` object
    pub fn read<'py>(&mut self, py: Python<'py>, size: usize) -> PyResult<Bound<'py, PyByteArray>> {
        // Resize the buffer to the requested size
        self.buffer.resize(size, 0);

        // Perform the read operation into the internal buffer
        match self.reader.read(&mut self.buffer) {
            Ok(bytes_read) => unsafe {
                // Truncate buffer to actual read size.
                self.buffer.truncate(bytes_read);

                // Check if we already have a PyByteArray stored
                if let Some(py_bytearray) = &self.py_bytes {
                    // Clone the Py reference and bind it to the current Python context
                    let byte_array = py_bytearray.clone_ref(py).into_bound(py);

                    // Resize the bytearray to fit the actual number of bytes read
                    byte_array.resize(bytes_read)?;
                    // Update the PyByteArray with the new buffer
                    byte_array.as_bytes_mut().copy_from_slice(&self.buffer);

                    Ok(byte_array)
                } else {
                    // Create a new PyByteArray from the buffer
                    let new_byte_array = PyByteArray::new_bound(py, self.buffer.as_slice());
                    self.py_bytes = Some(new_byte_array.clone().unbind());

                    Ok(new_byte_array)
                }
            },
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
        Self(ecore::Extractor::new())
    }

    /// Set the maximum length of the extracted text. Used only for extract_to_string functions
    /// Default: 500_000
    pub fn set_extract_string_max_length(&self, max_length: i32) -> Self {
        let inner = self.0.clone().set_extract_string_max_length(max_length);
        Self(inner)
    }

    /// Set the encoding to use for when extracting text to a stream.
    /// Not used for extract_to_string functions.
    /// Default: CharSet::UTF_8
    pub fn set_encoding(&self, encoding: CharSet) -> PyResult<Self> {
        let inner = self.0.clone().set_encoding(encoding.into());
        Ok(Self(inner))
    }

    /// Set the configuration for the PDF parser
    pub fn set_pdf_config(&self, config: PdfParserConfig) -> PyResult<Self> {
        let inner = self.0.clone().set_pdf_config(config.into());
        Ok(Self(inner))
    }

    /// Set the configuration for the Office parser
    pub fn set_office_config(&self, config: OfficeParserConfig) -> PyResult<Self> {
        let inner = self.0.clone().set_office_config(config.into());
        Ok(Self(inner))
    }

    /// Set the configuration for the Tesseract OCR
    pub fn set_ocr_config(&self, config: TesseractOcrConfig) -> PyResult<Self> {
        let inner = self.0.clone().set_ocr_config(config.into());
        Ok(Self(inner))
    }

    /// Extracts text from a file path. Returns a stream of the extracted text
    /// the stream is decoded using the extractor's `encoding`
    pub fn extract_file(&self, filename: &str) -> PyResult<StreamReader> {
        let reader = self
            .0
            .extract_file(filename)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        // Create a new `StreamReader` with initial buffer capacity of ecore::DEFAULT_BUF_SIZE bytes
        Ok(StreamReader {
            reader,
            buffer: Vec::with_capacity(ecore::DEFAULT_BUF_SIZE),
            py_bytes: None,
        })
    }

    /// Extracts text from a file path. Returns a string that is of maximum length
    /// of the extractor's `extract_string_max_length`
    pub fn extract_file_to_string(&self, filename: &str) -> PyResult<String> {
        Ok(self.0
            .extract_file_to_struct(filename)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?.content)
    }

    /// Extracts text and metadata from a file path. Returns a dict that contains the content and metadata.
    pub fn extract_file_to_dict(&self, filename: &str) -> PyResult<PyObject> {
        let extract_struct = self.0
            .extract_file_to_struct(filename)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        let content = extract_struct.content;
        let metadata = extract_struct.metadata;

        Python::with_gil(|py| {
            let dict = vec![
                ("content", content.into_py(py)),
                ("metadata", PyList::new_bound(py, &metadata).into_py(py))
            ].into_py_dict_bound(py);
            Ok(dict.into())
        })
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
