use algo::BitMap;
use pyo3::prelude::*;

#[pyclass(name = "BitMap")]
pub struct PyBitMap {
    inner: BitMap,
}
#[pymethods]
impl PyBitMap {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: BitMap::new(),
        }
    }
    pub fn add(&mut self, value: u32) {
        self.inner.add(value);
    }
    pub fn remove(&mut self, value: u32) {
        self.inner.remove(value);
    }
    pub fn contains(&self, value: u32) -> bool {
        self.inner.contains(value)
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn len(&self) -> u64 {
        self.inner.len()
    }
    pub fn is_disjoint(&self, other: &PyBitMap) -> bool {
        self.inner.is_disjoint(&other.inner)
    }
    pub fn is_subset(&self, other: &PyBitMap) -> bool {
        self.inner.is_subset(&other.inner)
    }
    pub fn is_superset(&self, other: &PyBitMap) -> bool {
        self.inner.is_superset(&other.inner)
    }

    pub fn display(&self) -> String {
        format!("{:?}", self.inner)
    }
    pub fn __repr__(&self) -> String {
        format!("{}", self)
    }
}

impl std::fmt::Display for PyBitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<BitMap at: {:?}>", self.inner)
    }
}
impl Default for PyBitMap {
    fn default() -> Self {
        Self::new()
    }
}
