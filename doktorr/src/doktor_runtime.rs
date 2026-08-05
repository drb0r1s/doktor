use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use doktorc::middleend::shaper_ast::{Location, TextMeasurement, ImageMeasurement, ShaperDoktorNode};
use doktorc::middleend::shaper::Shaper;
use doktorc::middleend::scroller::Scroller;
use doktorc::middleend::painter_ast::DrawStructure;
use doktorc::middleend::painter::Painter;

use doktorc::backend::packer::Packer;

use crate::parsed_doktorb::ParsedDoktorb;

#[wasm_bindgen]
pub struct DoktorRuntime {
    scroll_offsets: HashMap<String, Location>,
    latest_shaper_doktor_node: Option<ShaperDoktorNode>,
}

#[wasm_bindgen]
impl DoktorRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        DoktorRuntime {
            scroll_offsets: HashMap::new(),
            latest_shaper_doktor_node: None,
        }
    }

    #[wasm_bindgen(js_name=compile)]
    pub fn compile(&mut self, written_doktorb: &[u8], viewport_width: f32, viewport_height: f32, js_text_measurements: JsValue, js_image_measurements: JsValue) -> Result<ParsedDoktorb, JsValue> {
        let resolver_doktor_node = bincode::deserialize(written_doktorb).map_err(|e| JsValue::from_str(&format!("Failed to deserialize: {e}")))?;

        let text_measurements: Vec<TextMeasurement> = serde_wasm_bindgen::from_value(js_text_measurements).map_err(|e| JsValue::from_str(&format!("Failed to parse measurements: {e}")))?;
        let image_measurements: Vec<ImageMeasurement> = serde_wasm_bindgen::from_value(js_image_measurements).map_err(|e| JsValue::from_str(&format!("Failed to parse measurements: {e}")))?;

        let shaper_doktor_node = Shaper::new(viewport_width, viewport_height).shape(resolver_doktor_node, &text_measurements, &image_measurements);
        let scroller_doktor_node = Scroller::new().scroll(&shaper_doktor_node, &self.scroll_offsets);
        let draw_structures = Painter::new().paint(scroller_doktor_node);

        self.latest_shaper_doktor_node = Some(shaper_doktor_node);

        Ok(Self::pack(&draw_structures))
    }

    #[wasm_bindgen(js_name = updateScrollOffset)]
    pub fn update_scroll_offset(&mut self, tag: String, x: f32, y: f32) -> Result<ParsedDoktorb, JsValue> {
        self.scroll_offsets.insert(tag, Location { x, y });

        let shaper_doktor_node = self.latest_shaper_doktor_node.as_ref().ok_or_else(|| JsValue::from_str("No prior layout to scroll"))?;

        let scroller_doktor_node = Scroller::new().scroll(shaper_doktor_node, &self.scroll_offsets);
        let draw_structures = Painter::new().paint(scroller_doktor_node);

        Ok(Self::pack(&draw_structures))
    }

    fn pack(draw_structures: &Vec<DrawStructure>) -> ParsedDoktorb {
        let packed_packets = Packer::new().pack(draw_structures);

        ParsedDoktorb::new(packed_packets.numeric_buffer, packed_packets.string_table)
    }

    #[wasm_bindgen(js_name = getBlock)]
    pub fn get_block(&self, id: u32) -> Result<JsValue, JsValue> {
        let shaper_doktor_node: ShaperDoktorNode = self.latest_shaper_doktor_node.as_ref().ok_or_else(|| JsValue::from_str("No prior layout available"))?;
        let found_block: ShaperBlockNode = shaper_doktor_node.children.iter().find_map(|child| Self::find_block(child, id));

        match found {
            Some(block) => serde_wasm_bindgen::to_value(block).map_err(|e| JsValue::from_str(&format!("Failed to serialize block: {e}"))),
            None => Ok(JsValue::NULL),
        }
    }

    fn find_block(node: &ShaperBlockNode, id: u32) -> Option<&ShaperBlockNode> {
        if node.id == id {
            return Some(node);
        }

        node.children.iter().find_map(|child| Self::find_block(child, id))
    }
}