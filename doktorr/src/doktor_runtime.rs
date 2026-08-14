use std::collections::HashMap;
use uuid::Uuid;

use wasm_bindgen::prelude::*;
use web_sys::console;

use doktorc::frontend::resolver::ast::system_styles::Overflow;

use doktorc::middleend::shaper_ast::{Location, Clip, TextMeasurement, ImageMeasurement, ShaperBlockNode, ShaperDoktorNode};
use doktorc::middleend::shaper::Shaper;
use doktorc::middleend::scroller::Scroller;
use doktorc::middleend::painter::Painter;

use doktorc::backend::packer::Packer;

use crate::parsed_doktorb::ParsedDoktorb;

#[wasm_bindgen]
pub struct DoktorRuntime {
    viewport_width: f32,
    viewport_height: f32,
    scroll_offsets: HashMap<Uuid, Location>,
    latest_shaper_doktor_node: Option<ShaperDoktorNode>,
}

#[wasm_bindgen]
impl DoktorRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        DoktorRuntime {
            viewport_width: 0.0,
            viewport_height: 0.0,
            scroll_offsets: HashMap::new(),
            latest_shaper_doktor_node: None,
        }
    }

    #[wasm_bindgen(js_name=execute)]
    pub fn execute(&mut self, written_doktorb: &[u8], viewport_width: f32, viewport_height: f32, js_text_measurements: JsValue, js_image_measurements: JsValue) -> Result<ParsedDoktorb, JsValue> {
        console::log_7(
            &JsValue::from_str("%c[%cDOKTOR%c Runtime]%c Executing DOKTOR Binary %c(compiled.doktorb)%c..."),
            &JsValue::from_str("color: orange;"),
            &JsValue::from_str("color: orange; font-weight: bold;"),
            &JsValue::from_str("color: orange;"),
            &JsValue::from_str("color: inherit;"),
            &JsValue::from_str("font-style: italic;"),
            &JsValue::from_str("font-style: normal;"),
        );
        
        self.viewport_width = viewport_width;
        self.viewport_height = viewport_height;
        
        let resolver_doktor_node = bincode::deserialize(written_doktorb).map_err(|e| JsValue::from_str(&format!("Failed to deserialize: {e}")))?;

        console::log_6(
            &JsValue::from_str("%c[%cDOKTOR%c Runtime]%c (Resolver)%c AST has been deserialized."),
            &JsValue::from_str("color: orange;"),
            &JsValue::from_str("color: orange; font-weight: bold;"),
            &JsValue::from_str("color: orange;"),
            &JsValue::from_str("color: orange; font-style: italic;"),
            &JsValue::from_str("color: inherit;"),
        );

        let text_measurements: Vec<TextMeasurement> = serde_wasm_bindgen::from_value(js_text_measurements).map_err(|e| JsValue::from_str(&format!("Failed to parse measurements: {e}")))?;
        let image_measurements: Vec<ImageMeasurement> = serde_wasm_bindgen::from_value(js_image_measurements).map_err(|e| JsValue::from_str(&format!("Failed to parse measurements: {e}")))?;

        let shaper_doktor_node = Shaper::new(viewport_width, viewport_height).shape(resolver_doktor_node, &text_measurements, &image_measurements);
        
        console::log_6(
            &JsValue::from_str("%c[%cDOKTOR%c Runtime]%c (Shaper)%c AST has been built."),
            &JsValue::from_str("color: orange;"),
            &JsValue::from_str("color: orange; font-weight: bold;"),
            &JsValue::from_str("color: orange;"),
            &JsValue::from_str("color: orange; font-style: italic;"),
            &JsValue::from_str("color: inherit;"),
        );

        let parsed_doktorb = self.finalize(&shaper_doktor_node, true);
        
        self.latest_shaper_doktor_node = Some(shaper_doktor_node);

        Ok(parsed_doktorb)
    }

    #[wasm_bindgen(js_name = updateScrollOffset)]
    pub fn update_scroll_offset(&mut self, string_id: String, x: f32, y: f32) -> Result<ParsedDoktorb, JsValue> {
        let id: Uuid = Uuid::parse_str(&string_id).map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.scroll_offsets.insert(id, Location { x, y });

        let shaper_doktor_node = self.latest_shaper_doktor_node.as_ref().ok_or_else(|| JsValue::from_str("No prior layout to scroll"))?;
        Ok(self.finalize(shaper_doktor_node, false))
    }

    fn finalize(&self, shaper_doktor_node: &ShaperDoktorNode, console_log: bool) -> ParsedDoktorb {
        let viewport_clip: Clip = Clip {
            x: (0.0, self.viewport_width),
            y: (0.0, self.viewport_height),
        };

        let scroller_doktor_node = Scroller::new().scroll(shaper_doktor_node, viewport_clip, &self.scroll_offsets);

        if console_log {
            console::log_6(
                &JsValue::from_str("%c[%cDOKTOR%c Runtime]%c (Scroller)%c AST has been built."),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: orange; font-weight: bold;"),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: orange; font-style: italic;"),
                &JsValue::from_str("color: inherit;"),
            );
        }

        let draw_structures = Painter::new().paint(scroller_doktor_node);

        if console_log {
            console::log_6(
                &JsValue::from_str("%c[%cDOKTOR%c Runtime]%c (Painter)%c Scroller's AST has been converted to drawing structures."),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: orange; font-weight: bold;"),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: orange; font-style: italic;"),
                &JsValue::from_str("color: inherit;"),
            );
        }

        let packed_packets = Packer::new().pack(&draw_structures);

        if console_log {
            console::log_6(
                &JsValue::from_str("%c[%cDOKTOR%c Runtime]%c (Packer)%c Drawing structures have been packed."),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: orange; font-weight: bold;"),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: orange; font-style: italic;"),
                &JsValue::from_str("color: inherit;"),
            );

            console::log_7(
                &JsValue::from_str("%c[%cDOKTOR%c Runtime]%c DOKTOR Binary %c(compiled.doktorb)%c has been executed."),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: orange; font-weight: bold;"),
                &JsValue::from_str("color: orange;"),
                &JsValue::from_str("color: inherit;"),
                &JsValue::from_str("font-style: italic;"),
                &JsValue::from_str("font-style: normal;"),
            );
        }

        ParsedDoktorb::new(packed_packets.numeric_buffer, packed_packets.string_table)
    }

    #[wasm_bindgen(js_name = getBlock)]
    pub fn get_block(&self, x: f32, y: f32) -> Result<JsValue, JsValue> {
        let shaper_doktor_node: &ShaperDoktorNode = self.latest_shaper_doktor_node.as_ref().ok_or_else(|| JsValue::from_str("No prior layout available"))?;

        let found_block: Option<&ShaperBlockNode> = shaper_doktor_node.children.iter().find_map(|child| Self::find_block(child, x, y));

        match found_block {
            Some(block) => serde_wasm_bindgen::to_value(block).map_err(|e| JsValue::from_str(&format!("Failed to serialize block: {e}"))),
            None => Ok(JsValue::NULL),
        }
    }

    fn find_block(block: &ShaperBlockNode, x: f32, y: f32) -> Option<&ShaperBlockNode> {
        // Here we apply botttom-up finding approach so that the most specific child wins.
        if let Some(found_block) = block.children.iter().find_map(|child| Self::find_block(child, x, y)) {
            return Some(found_block);
        }

        if Self::is_block_target(block, x, y) {
            return Some(block);
        }

        None
    }

    #[wasm_bindgen(js_name = getScrollableAncestor)]
    pub fn get_scrollable_ancestor(&self, x: f32, y: f32) -> Result<JsValue, JsValue> {
        let shaper_doktor_node: &ShaperDoktorNode = self.latest_shaper_doktor_node.as_ref().ok_or_else(|| JsValue::from_str("No prior layout available"))?;
        let found_block = shaper_doktor_node.children.iter().find_map(|child| Self::find_scrollable_ancestor(child, x, y, None));

        match found_block {
            Some(block) => serde_wasm_bindgen::to_value(block).map_err(|e| JsValue::from_str(&format!("Failed to serialize block: {e}"))),
            None => Ok(JsValue::NULL),
        }
    }

    fn find_scrollable_ancestor<'a>(block: &'a ShaperBlockNode, x: f32, y: f32, nearest_scrollable: Option<&'a ShaperBlockNode>) -> Option<&'a ShaperBlockNode> {
        if !Self::is_block_target(block, x, y) {
            return None;
        }

        let is_scrollable = block.system_styles.get_unambiguous_overflow("x") == Overflow::Scroll || block.system_styles.get_unambiguous_overflow("y") == Overflow::Scroll;

        let carried = if is_scrollable { Some(block) } else { nearest_scrollable };

        // Deepest scrollable block will be scrolled, we need to check the children.
        if let Some(found) = block.children.iter().find_map(|child| Self::find_scrollable_ancestor(child, x, y, carried)) {
            return Some(found);
        }

        carried
    }

    fn is_block_target(node: &ShaperBlockNode, x: f32, y: f32) -> bool {
        let is_within_bounds: bool = x >= node.location.x && x <= node.location.x + node.size.width && y >= node.location.y && y <= node.location.y + node.size.height;
        let is_within_clip: bool = x >= node.clip.x.0 && x <= node.clip.x.1 && y >= node.clip.y.0 && y <= node.clip.y.1;

        is_within_bounds && is_within_clip
    }
}