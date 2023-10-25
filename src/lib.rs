#![deny(clippy::all)]
#[macro_use]
extern crate napi_derive;

use algorithms_fourth::string::huff_man;
#[napi]
pub fn huff_man_encode(data: String) -> Vec<u8> {
  let encoder = huff_man::HuffmanEncode::new();
  encoder.compress(&data)
}
#[napi]
pub fn huff_man_decode(data:Vec<u8>) -> String {
    let mut decoder = huff_man::HuffmanDecode::new(data);
    decoder.expand()
}
