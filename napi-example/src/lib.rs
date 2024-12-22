#![deny(clippy::all)]

mod algo;
mod matrix;

use napi_derive::napi;

#[napi]
pub fn plus(input: u32, input1: u32) -> u32 {
  input + input1
}
