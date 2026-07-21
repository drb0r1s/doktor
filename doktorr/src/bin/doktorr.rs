use wasm_bindgen::prelude::*;
use js_sys::{Float32Array, Uint8Array};

use doktorc::backend::packer::PACKET_SIZE;

const SIGNATURE: &[u8; 8] = b"DOKTORB0";

#[wasm_bindgen]
pub struct ParsedDoktorb {
    numeric_buffer: Vec<f32>,
    string_table: Vec<u8>,
}

#[wasm_bindgen]
impl ParsedDoktorb {
    #[wasm_bindgen(js_name = numericBuffer)]
    pub fn numeric_buffer(&self) -> Float32Array {
        Float32Array::from(self.numeric_buffer.as_slice())
    }

    #[wasm_bindgen(js_name = stringTable)]
    pub fn string_table(&self) -> Uint8Array {
        Uint8Array::from(self.string_table.as_slice())
    }
}

#[wasm_bindgen(js_name = parseDoktorb)]
pub fn parse_doktorb(bytes: &[u8]) -> Result<ParsedDoktorb, JsValue> {
    if bytes.len() < SIGNATURE.len() + 4 + 4 {
        return Err(JsValue::from_str("File too short to be a valid .doktorb file"));
    }

    let signature: &[u8] = &bytes[0..SIGNATURE.len()];

    if signature != SIGNATURE {
        return Err(JsValue::from_str("Invalid .doktorb signature"));
    }

    let mut cursor: usize = SIGNATURE.len();

    let draw_structures_count: u32 = read_u32_le(bytes, cursor)?;
    cursor += 4;

    let string_table_length: u32 = read_u32_le(bytes, cursor)?;
    cursor += 4;

    let numeric_buffer_element_count: usize = draw_structures_count as usize * PACKET_SIZE;
    let numeric_buffer_byte_length: usize = numeric_buffer_element_count * 4;

    if bytes.len() < cursor + numeric_buffer_byte_length + string_table_length as usize {
        return Err(JsValue::from_str("File shorter than header declares"));
    }

    let numeric_buffer: Vec<f32> = bytes[cursor..cursor + numeric_buffer_byte_length]
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    cursor += numeric_buffer_byte_length;

    let string_table: Vec<u8> = bytes[cursor..cursor + string_table_length as usize].to_vec();

    Ok(ParsedDoktorb {
        numeric_buffer,
        string_table,
    })
}

fn read_u32_le(bytes: &[u8], offset: usize) -> Result<u32, JsValue> {
    let slice: &[u8] = bytes.get(offset..offset + 4)
        .ok_or_else(|| JsValue::from_str("Unexpected end of file while reading header"))?;

    Ok(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}