#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn compress_to_base64(input: String) -> String {
  lz_str::compress_to_base64(&input)
}
