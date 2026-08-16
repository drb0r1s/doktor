use wasm_bindgen::prelude::*;
use serde::Serialize;

use doktorc::frontend::resolver::ast::nodes::{ResolverBlockNode, ResolverDoktorNode};

use doktorc::collections::font::Font;

// TEXT

#[derive(Serialize)]
struct TextMeasurementRequest {
    path: Vec<usize>,
    width: f32,
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
                width: node.system_styles.width,
                content: node.system_attributes.content.clone().unwrap_or_default(),
                content_size: node.system_styles.content_size,
                content_font: node.system_styles.content_font,
            });
        }

        collect_text_measurement_requests(&node.children, path, requests);
        path.pop();
    }
}

// IMAGE

#[derive(Serialize)]
struct ImageMeasurementRequest {
    path: Vec<usize>,
    source: String,
}

#[wasm_bindgen(js_name = getImageMeasurementRequests)]
pub fn get_image_measurement_requests(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let resolver_doktor_node: ResolverDoktorNode = bincode::deserialize(bytes).map_err(|e| JsValue::from_str(&format!("Failed to deserialize: {e}")))?;

    let mut image_measurement_requests: Vec<ImageMeasurementRequest> = Vec::new();
    collect_image_measurement_requests(&resolver_doktor_node.children, &mut Vec::new(), &mut image_measurement_requests);

    serde_wasm_bindgen::to_value(&image_measurement_requests).map_err(|e| JsValue::from_str(&format!("Serialization failed: {e}")))
}

fn collect_image_measurement_requests(nodes: &[ResolverBlockNode], path: &mut Vec<usize>, requests: &mut Vec<ImageMeasurementRequest>) {
    for (index, node) in nodes.iter().enumerate() {
        path.push(index);

        if node.block_type == "Image" {
            requests.push(ImageMeasurementRequest {
                path: path.clone(),
                source: node.system_attributes.source.clone().unwrap_or_default(),
            });
        }

        collect_image_measurement_requests(&node.children, path, requests);
        path.pop();
    }
}