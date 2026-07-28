use wasm_bindgen::prelude::*;
use serde::Serialize;

use doktorc::frontend::resolver_ast::{Font, ResolverBlockNode, ResolverDoktorNode};

#[derive(Serialize)]
struct TextMeasurementRequest {
    path: Vec<usize>,
    content: String,
    content_size: f32,
    content_font: Font,
}

#[wasm_bindgen(js_name = getTextMeasurementRequests)]
pub fn get_text_measurement_requests(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let resolver_doktor_node: ResolverDoktorNode = bincode::deserialize(bytes).map_err(|e| JsValue::from_str(&format!("Failed to deserialize: {e}")))?;

    let mut text_measurement_requests: Vec<TextMeasurementRequest> = Vec::new();
    collect_text_measurement_requests(&resolver_doktor_node.children, &mut Vec::new(), &mut text_measurement_requests);

    serde_wasm_bindgen::to_value(&text_measurement_requests).map_err(|e| JsValue::from_str(&format!("Serialization failed: {e}")))
}

fn collect_text_measurement_requests(nodes: &[ResolverBlockNode], path: &mut Vec<usize>, requests: &mut Vec<TextMeasurementRequest>) {
    for (index, node) in nodes.iter().enumerate() {
        path.push(index);

        if node.block_type == "Text" {
            requests.push(TextMeasurementRequest {
                path: path.clone(),
                content: node.system_attributes.content.clone().unwrap_or_default(),
                content_size: node.system_styles.content_size,
                content_font: node.system_styles.content_font,
            });
        }

        collect_text_measurement_requests(&node.children, path, requests);
        path.pop();
    }
}