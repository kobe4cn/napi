use pyo3::prelude::*;
mod bitmap;
mod hasher;
mod matrix;
use bitmap::PyBitMap;
use hasher::PyAlgo;
use matrix::PyMatrix;
/// Prints a message.
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello from python-binding!".into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_class::<PyAlgo>()?;
    m.add_class::<PyMatrix>()?;
    m.add_class::<PyBitMap>()?;
    Ok(())
}
