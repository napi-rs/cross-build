#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn compress_to_base64(input: String) -> String {
  lz_str::compress_to_base64(&input)
}
