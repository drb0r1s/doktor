use crate::frontend::resolver_ast::RGB;

use crate::middleend::painter_ast::{Rectangle, Text, Image, DrawStructure};

use crate::data::packet_structure;

pub struct PackedPackets {
    pub numeric_buffer: Vec<f32>,
    pub string_table: Vec<u8>,
}

pub struct Packer;

impl Packer {
    pub fn new() -> Self {
        Packer
    }

    pub fn pack(&self, draw_structures: &[DrawStructure]) -> PackedPackets {
        let mut numeric_buffer: Vec<f32> = vec![0.0; draw_structures.len() * packet_structure::PACKET_SIZE];
        let mut string_table: Vec<u8> = Vec::new();

        for (index, draw_structure) in draw_structures.iter().enumerate() {
            let row_start: usize = index * packet_structure::PACKET_SIZE;
            let row: &mut [f32] = &mut numeric_buffer[row_start..row_start + packet_structure::PACKET_SIZE];

            match draw_structure {
                DrawStructure::Rectangle(rectangle) => Self::pack_rectangle(rectangle, row),
                DrawStructure::Text(text) => Self::pack_text(text, row, &mut string_table),
                DrawStructure::Image(image) => Self::pack_image(image, row, &mut string_table),
            }
        }

        PackedPackets {
            numeric_buffer,
            string_table,
        }
    }

    fn pack_rectangle(rectangle: &Rectangle, row: &mut [f32]) {
        row[packet_structure::PACKET_TYPE] = packet_structure::PACKET_RECTANGLE_TYPE;

        row[packet_structure::PACKET_X] = rectangle.location.x;
        row[packet_structure::PACKET_Y] = rectangle.location.y;

        row[packet_structure::PACKET_WIDTH] = rectangle.width;
        row[packet_structure::PACKET_HEIGHT] = rectangle.height;

        row[packet_structure::PACKET_CLIP_X_START] = rectangle.clip.x.0;
        row[packet_structure::PACKET_CLIP_X_END] = rectangle.clip.x.1;
        row[packet_structure::PACKET_CLIP_Y_START] = rectangle.clip.y.0;
        row[packet_structure::PACKET_CLIP_Y_END] = rectangle.clip.y.1;

        row[packet_structure::PACKET_BACKGROUND_COLOR] = Self::pack_color(rectangle.background_color);
        row[packet_structure::PACKET_BACKGROUND_COLOR_ALPHA] = rectangle.background_color.a as f32;

        row[packet_structure::PACKET_BORDER_TOP_COLOR] = Self::pack_color(rectangle.border_top_color);
        row[packet_structure::PACKET_BORDER_TOP_COLOR_ALPHA] = rectangle.border_top_color.a as f32;
        row[packet_structure::PACKET_BORDER_TOP_SIZE] = rectangle.border_top_size;
        row[packet_structure::PACKET_BORDER_TOP_TYPE] = rectangle.border_top_type as u32 as f32;

        row[packet_structure::PACKET_BORDER_BOTTOM_COLOR] = Self::pack_color(rectangle.border_bottom_color);
        row[packet_structure::PACKET_BORDER_BOTTOM_COLOR_ALPHA] = rectangle.border_bottom_color.a as f32;
        row[packet_structure::PACKET_BORDER_BOTTOM_SIZE] = rectangle.border_bottom_size;
        row[packet_structure::PACKET_BORDER_BOTTOM_TYPE] = rectangle.border_bottom_type as u32 as f32;
        
        row[packet_structure::PACKET_BORDER_LEFT_COLOR] = Self::pack_color(rectangle.border_left_color);
        row[packet_structure::PACKET_BORDER_LEFT_COLOR_ALPHA] = rectangle.border_left_color.a as f32;
        row[packet_structure::PACKET_BORDER_LEFT_SIZE] = rectangle.border_left_size;
        row[packet_structure::PACKET_BORDER_LEFT_TYPE] = rectangle.border_left_type as u32 as f32;

        row[packet_structure::PACKET_BORDER_RIGHT_COLOR] = Self::pack_color(rectangle.border_right_color);
        row[packet_structure::PACKET_BORDER_RIGHT_COLOR_ALPHA] = rectangle.border_right_color.a as f32;
        row[packet_structure::PACKET_BORDER_RIGHT_SIZE] = rectangle.border_right_size;
        row[packet_structure::PACKET_BORDER_RIGHT_TYPE] = rectangle.border_right_type as u32 as f32;

        row[packet_structure::PACKET_OPACITY] = rectangle.opacity;
    }

    fn pack_text(text: &Text, row: &mut [f32], string_table: &mut Vec<u8>) {
        let (offset, length): (usize, usize) = Self::push_string(string_table, &text.content);

        row[packet_structure::PACKET_TYPE] = packet_structure::PACKET_TEXT_TYPE;

        row[packet_structure::PACKET_X] = text.location.x;
        row[packet_structure::PACKET_Y] = text.location.y;

        row[packet_structure::PACKET_CLIP_X_START] = text.clip.x.0;
        row[packet_structure::PACKET_CLIP_X_END] = text.clip.x.1;
        row[packet_structure::PACKET_CLIP_Y_START] = text.clip.y.0;
        row[packet_structure::PACKET_CLIP_Y_END] = text.clip.y.1;

        row[packet_structure::PACKET_BACKGROUND_COLOR] = Self::pack_color(text.background_color);
        row[packet_structure::PACKET_BACKGROUND_COLOR_ALPHA] = text.background_color.a as f32;

        row[packet_structure::PACKET_STRING_OFFSET] = offset as f32;
        row[packet_structure::PACKET_STRING_LENGTH] = length as f32;

        row[packet_structure::PACKET_CONTENT_COLOR] = Self::pack_color(text.content_color);
        row[packet_structure::PACKET_CONTENT_COLOR_ALPHA] = text.content_color.a as f32;
        row[packet_structure::PACKET_CONTENT_SIZE] = text.content_size;
        row[packet_structure::PACKET_CONTENT_FONT] = text.content_font as u32 as f32;

        row[packet_structure::PACKET_BORDER_TOP_COLOR] = Self::pack_color(text.border_top_color);
        row[packet_structure::PACKET_BORDER_TOP_COLOR_ALPHA] = text.border_top_color.a as f32;
        row[packet_structure::PACKET_BORDER_TOP_SIZE] = text.border_top_size;
        row[packet_structure::PACKET_BORDER_TOP_TYPE] = text.border_top_type as u32 as f32;

        row[packet_structure::PACKET_BORDER_BOTTOM_COLOR] = Self::pack_color(text.border_bottom_color);
        row[packet_structure::PACKET_BORDER_BOTTOM_COLOR_ALPHA] = text.border_bottom_color.a as f32;
        row[packet_structure::PACKET_BORDER_BOTTOM_SIZE] = text.border_bottom_size;
        row[packet_structure::PACKET_BORDER_BOTTOM_TYPE] = text.border_bottom_type as u32 as f32;
        
        row[packet_structure::PACKET_BORDER_LEFT_COLOR] = Self::pack_color(text.border_left_color);
        row[packet_structure::PACKET_BORDER_LEFT_COLOR_ALPHA] = text.border_left_color.a as f32;
        row[packet_structure::PACKET_BORDER_LEFT_SIZE] = text.border_left_size;
        row[packet_structure::PACKET_BORDER_LEFT_TYPE] = text.border_left_type as u32 as f32;

        row[packet_structure::PACKET_BORDER_RIGHT_COLOR] = Self::pack_color(text.border_right_color);
        row[packet_structure::PACKET_BORDER_RIGHT_COLOR_ALPHA] = text.border_right_color.a as f32;
        row[packet_structure::PACKET_BORDER_RIGHT_SIZE] = text.border_right_size;
        row[packet_structure::PACKET_BORDER_RIGHT_TYPE] = text.border_right_type as u32 as f32;

        row[packet_structure::PACKET_OPACITY] = text.opacity;
    }

    fn pack_image(image: &Image, row: &mut [f32], string_table: &mut Vec<u8>) {
        let (offset, length): (usize, usize) = Self::push_string(string_table, &image.source);

        row[packet_structure::PACKET_TYPE] = packet_structure::PACKET_IMAGE_TYPE;

        row[packet_structure::PACKET_X] = image.location.x;
        row[packet_structure::PACKET_Y] = image.location.y;

        row[packet_structure::PACKET_WIDTH] = image.width;
        row[packet_structure::PACKET_HEIGHT] = image.height;

        row[packet_structure::PACKET_CLIP_X_START] = image.clip.x.0;
        row[packet_structure::PACKET_CLIP_X_END] = image.clip.x.1;
        row[packet_structure::PACKET_CLIP_Y_START] = image.clip.y.0;
        row[packet_structure::PACKET_CLIP_Y_END] = image.clip.y.1;

        row[packet_structure::PACKET_BACKGROUND_COLOR] = Self::pack_color(image.background_color);
        row[packet_structure::PACKET_BACKGROUND_COLOR_ALPHA] = image.background_color.a as f32;

        row[packet_structure::PACKET_STRING_OFFSET] = offset as f32;
        row[packet_structure::PACKET_STRING_LENGTH] = length as f32;
        
        row[packet_structure::PACKET_BORDER_TOP_COLOR] = Self::pack_color(image.border_top_color);
        row[packet_structure::PACKET_BORDER_TOP_COLOR_ALPHA] = image.border_top_color.a as f32;
        row[packet_structure::PACKET_BORDER_TOP_SIZE] = image.border_top_size;
        row[packet_structure::PACKET_BORDER_TOP_TYPE] = image.border_top_type as u32 as f32;

        row[packet_structure::PACKET_BORDER_BOTTOM_COLOR] = Self::pack_color(image.border_bottom_color);
        row[packet_structure::PACKET_BORDER_BOTTOM_COLOR_ALPHA] = image.border_bottom_color.a as f32;
        row[packet_structure::PACKET_BORDER_BOTTOM_SIZE] = image.border_bottom_size;
        row[packet_structure::PACKET_BORDER_BOTTOM_TYPE] = image.border_bottom_type as u32 as f32;
        
        row[packet_structure::PACKET_BORDER_LEFT_COLOR] = Self::pack_color(image.border_left_color);
        row[packet_structure::PACKET_BORDER_LEFT_COLOR_ALPHA] = image.border_left_color.a as f32;
        row[packet_structure::PACKET_BORDER_LEFT_SIZE] = image.border_left_size;
        row[packet_structure::PACKET_BORDER_LEFT_TYPE] = image.border_left_type as u32 as f32;

        row[packet_structure::PACKET_BORDER_RIGHT_COLOR] = Self::pack_color(image.border_right_color);
        row[packet_structure::PACKET_BORDER_RIGHT_COLOR_ALPHA] = image.border_right_color.a as f32;
        row[packet_structure::PACKET_BORDER_RIGHT_SIZE] = image.border_right_size;
        row[packet_structure::PACKET_BORDER_RIGHT_TYPE] = image.border_right_type as u32 as f32;

        row[packet_structure::PACKET_OPACITY] = image.opacity;
    }

    fn pack_color(color: RGB) -> f32 {
        let packed: u32 = (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32);
        packed as f32
    }

    fn push_string(string_table: &mut Vec<u8>, value: &str) -> (usize, usize) {
        let offset: usize = string_table.len();
        let bytes: &[u8] = value.as_bytes();

        string_table.extend_from_slice(bytes);

        (offset, bytes.len())
    }
}