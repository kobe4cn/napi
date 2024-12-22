use core::fmt;

use algo::{Algo, AlgoType};

use pyo3::prelude::*;

#[pyclass(name = "Algo")]
pub struct PyAlgo {
    inner: Algo,
}

#[pymethods]
impl PyAlgo {
    #[new]
    #[pyo3(signature = (name=""))]
    pub fn new(name: &str) -> Self {
        let algo = match name {
            "blake3" => AlgoType::Blake3,
            _ => AlgoType::Default,
        };
        Self {
            inner: Algo::new(algo),
        }
    }

    pub fn hash(&self, v: &str) -> String {
        self.inner.hash(v.to_string())
    }
    pub fn get_name(&self) -> String {
        self.inner.get_name().to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for PyAlgo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Algo at: {}>", self.inner.get_name())
    }
}
