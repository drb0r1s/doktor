use wasm_bindgen::prelude::*;
use js_sys::{Float32Array, Uint8Array};

#[wasm_bindgen]
pub struct ParsedDoktorb {
    numeric_buffer: Vec<f32>,
    string_table: Vec<u8>,
}

#[wasm_bindgen]
impl ParsedDoktorb {
    #[wasm_bindgen(constructor)]
    pub fn new(numeric_buffer: Vec<f32>, string_table: Vec<u8>) -> ParsedDoktorb {
        ParsedDoktorb {
            numeric_buffer,
            string_table,
        }
    }

    #[wasm_bindgen(js_name = numericBuffer)]
    pub fn numeric_buffer(&self) -> Float32Array {
        Float32Array::from(self.numeric_buffer.as_slice())
    }

    #[wasm_bindgen(js_name = stringTable)]
    pub fn string_table(&self) -> Uint8Array {
        Uint8Array::from(self.string_table.as_slice())
    }
}