use crate::{ecore, OfficeParserConfig, PdfParserConfig, TesseractOcrConfig};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::types::PyDict;
use std::collections::HashMap;
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

    /// Reads into the specified buffer
    pub fn readinto<'py>(&mut self, buf: Bound<'py, PyByteArray>) -> PyResult<usize> {
        let bs = unsafe { buf.as_bytes_mut() };

        let bytes_read = self
            .reader
            .read(bs)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        Ok(bytes_read)
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

    /// Set the configuration for the parse as xml
    pub fn set_xml_output(&self, xml_output: bool) -> PyResult<Self> {
        let inner = self.0.clone().set_xml_output(xml_output);
        Ok(Self(inner))
    }

    /// Extracts text from a file path. Returns a tuple with stream of the extracted text
    /// the stream is decoded using the extractor's `encoding` and tika metadata.
    pub fn extract_file<'py>(
        &self,
        filename: &str,
        py: Python<'py>,
    ) -> PyResult<(StreamReader, PyObject)> {
        let (reader, metadata) = self
            .0
            .extract_file(filename)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        // Create a new `StreamReader` with initial buffer capacity of ecore::DEFAULT_BUF_SIZE bytes
        let py_metadata = metadata_hashmap_to_pydict(py, &metadata)?;
        Ok((
            StreamReader {
                reader,
                buffer: Vec::with_capacity(ecore::DEFAULT_BUF_SIZE),
                py_bytes: None,
            },
            py_metadata.into(),
        ))
    }

    /// Extracts text from a bytearray. Returns a tuple with stream of the extracted text
    /// the stream is decoded using the extractor's `encoding` and tika metadata.
    pub fn extract_bytes<'py>(
        &self,
        buffer: &Bound<'_, PyByteArray>,
        py: Python<'py>,
    ) -> PyResult<(StreamReader, PyObject)> {
        let slice = buffer.to_vec();
        let (reader, metadata) = self
            .0
            .extract_bytes(&slice)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        // Create a new `StreamReader` with initial buffer capacity of ecore::DEFAULT_BUF_SIZE bytes
        let py_metadata = metadata_hashmap_to_pydict(py, &metadata)?;
        Ok((
            StreamReader {
                reader,
                buffer: Vec::with_capacity(ecore::DEFAULT_BUF_SIZE),
                py_bytes: None,
            },
            py_metadata.into(),
        ))
    }

    /// Extracts text from a url. Returns a tuple with string that is of maximum length
    /// of the extractor's `extract_string_max_length` and tika metdata.
    pub fn extract_url<'py>(
        &self,
        url: &str,
        py: Python<'py>,
    ) -> PyResult<(StreamReader, PyObject)> {
        let (reader, metadata) = self
            .0
            .extract_url(&url)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        // Create a new `StreamReader` with initial buffer capacity of ecore::DEFAULT_BUF_SIZE bytes
        let py_metadata = metadata_hashmap_to_pydict(py, &metadata)?;
        Ok((
            StreamReader {
                reader,
                buffer: Vec::with_capacity(ecore::DEFAULT_BUF_SIZE),
                py_bytes: None,
            },
            py_metadata.into(),
        ))
    }

    /// Extracts text from a file path. Returns a tuple with string that is of maximum length
    /// of the extractor's `extract_string_max_length` and the metadata as dict.
    pub fn extract_file_to_string<'py>(
        &self,
        filename: &str,
        py: Python<'py>,
    ) -> PyResult<(String, PyObject)> {
        let (content, metadata) = self
            .0
            .extract_file_to_string(filename)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        let py_metadata = metadata_hashmap_to_pydict(py, &metadata)?;
        Ok((content, py_metadata.into()))
    }

    /// Extracts text from a bytearray. string that is of maximum length
    /// of the extractor's `extract_string_max_length` and the metadata as dict.
    pub fn extract_bytes_to_string<'py>(
        &self,
        buffer: &Bound<'_, PyByteArray>,
        py: Python<'py>,
    ) -> PyResult<(String, PyObject)> {
        let (content, metadata) = self
            .0
            .extract_bytes_to_string(&buffer.to_vec())
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        // Create a new `StreamReader` with initial buffer capacity of ecore::DEFAULT_BUF_SIZE bytes
        let py_metadata = metadata_hashmap_to_pydict(py, &metadata)?;
        Ok((content, py_metadata.into()))
    }

    /// Extracts text from a URL. Returns a tuple with string that is of maximum length
    /// of the extractor's `extract_string_max_length` and the metadata as dict.
    pub fn extract_url_to_string<'py>(
        &self,
        url: &str,
        py: Python<'py>,
    ) -> PyResult<(String, PyObject)> {
        let (content, metadata) = self
            .0
            .extract_url_to_string(url)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;

        let py_metadata = metadata_hashmap_to_pydict(py, &metadata)?;
        Ok((content, py_metadata.into()))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

/// Converts HashMap<String, Vec<String> to PyDict
fn metadata_hashmap_to_pydict<'py>(
    py: Python<'py>,
    hashmap: &HashMap<String, Vec<String>>,
) -> Result<Bound<'py, PyDict>, PyErr> {
    let pydict = PyDict::new_bound(py);
    for (key, value) in hashmap {
        pydict
            .set_item(key, value)
            .map_err(|e| PyErr::new::<PyTypeError, _>(format!("{:?}", e)))?;
    }
    Ok(pydict)
}
