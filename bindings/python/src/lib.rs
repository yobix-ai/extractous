use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

/// extract content of file
#[pyfunction]
fn extract(filename: &str) -> PyResult<String> {
    match extract_rs::extract(filename) {
        Ok(content) => Ok(content),
        Err(e) => Err(PyErr::new::<PyTypeError, _>(format!("{:?}", e)))
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _extractrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    Ok(())
}