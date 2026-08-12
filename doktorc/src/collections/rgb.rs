use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGB {
    pub fn hex_to_rgb(value: &str) -> Option<RGB> {
        let hex = value.strip_prefix('#')?;

        if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        match hex.len() {
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

                Some(RGB { r, g, b, a: 255 })
            }

            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;

                Some(RGB { r, g, b, a })
            }

            _ => None
        }
    }
}