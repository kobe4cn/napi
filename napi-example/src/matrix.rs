use algo::{multiply, Matrix};
use napi::{Error, Status};
use napi_derive::napi;

#[napi(js_name = "Matrix")]
#[derive(Debug)]
pub struct JsMatrix {
  inner: Matrix<f64>,
}

#[napi]
impl JsMatrix {
  #[napi(constructor)]
  pub fn new(data: Vec<Vec<f64>>) -> napi::Result<Self> {
    if data.is_empty() {
      return Err(Error::new(Status::InvalidArg, "Data cannot be empty"));
    }
    let rows = data.len();
    let cols = data[0].len();
    let data: Vec<_> = data.into_iter().flatten().collect();
    Ok(Self {
      inner: Matrix::new(rows, cols, data),
    })
  }

  #[napi]
  pub fn mult(&self, other: Vec<Vec<f64>>) -> napi::Result<JsMatrix> {
    if other.is_empty() {
      return Err(Error::new(Status::InvalidArg, "Data cannot be empty"));
    }

    let other = JsMatrix::new(other)?;
    let result = multiply(&self.inner, &other.inner).unwrap();
    Ok(Self { inner: result })
  }

  #[napi]
  pub fn multiply(&self, other: &JsMatrix) -> napi::Result<JsMatrix> {
    let result = multiply(&self.inner, &other.inner).unwrap();
    Ok(Self { inner: result })
  }

  #[napi]
  pub fn display(&self) -> String {
    format!("{}", self.inner)
  }
}
