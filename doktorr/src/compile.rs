use wasm_bindgen::prelude::*;
use js_sys::{Float32Array, Uint8Array};

use doktorc::middleend::shaper_ast::TextMeasurement;
use doktorc::middleend::shaper::Shaper;
use doktorc::middleend::painter::Painter;

use doktorc::backend::packer::Packer;

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

#[wasm_bindgen(js_name=compile)]
pub fn compile(written_doktorb: &[u8], viewport_width: f32, viewport_height: f32, js_text_measurements: JsValue) -> Result<ParsedDoktorb, JsValue> {
    let resolver_doktor_node = bincode::deserialize(written_doktorb).map_err(|e| JsValue::from_str(&format!("Failed to deserialize: {e}")))?;

    let text_measurements: Vec<TextMeasurement> = serde_wasm_bindgen::from_value(js_text_measurements).map_err(|e| JsValue::from_str(&format!("Failed to parse measurements: {e}")))?;

    let shaper_doktor_node = Shaper::new(viewport_width, viewport_height).shape(resolver_doktor_node, &text_measurements);
    let draw_structures = Painter::new().paint(shaper_doktor_node);

    let packed_packets = Packer::new().pack(&draw_structures);

    Ok(ParsedDoktorb {
        numeric_buffer: packed_packets.numeric_buffer,
        string_table: packed_packets.string_table,
    })
}