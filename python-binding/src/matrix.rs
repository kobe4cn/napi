use core::fmt;

use algo::{multiply, Matrix};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass(name = "Matrix")]
pub struct PyMatrix {
    inner: Matrix<f64>,
}

#[pymethods]

impl PyMatrix {
    #[new]
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let data: Vec<_> = data.into_iter().flatten().collect();
        Self {
            inner: Matrix::new(rows, cols, data),
        }
    }

    pub fn mult(&self, other: Vec<Vec<f64>>) -> PyResult<PyMatrix> {
        if other.is_empty() {
            return Err(PyValueError::new_err("Data cannot be empty"));
        }

        let other = PyMatrix::new(other);
        let result = multiply(&self.inner, &other.inner).unwrap();
        Ok(PyMatrix { inner: result })
    }
    pub fn multiply(&self, other: &PyMatrix) -> PyResult<PyMatrix> {
        let result = multiply(&self.inner, &other.inner).unwrap();
        Ok(PyMatrix { inner: result })
    }

    pub fn display(&self) -> String {
        format!("{}", self.inner)
    }
    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Matrix at :{}>", self.inner)
    }
}
