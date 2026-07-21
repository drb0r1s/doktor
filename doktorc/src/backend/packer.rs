use crate::middleend::painter_ast::{Rectangle, Text, Image, DrawStructure};

pub const PACKET_SIZE: usize = 16;

const PACKET_TYPE: usize = 0;
const PACKET_X: usize = 1;
const PACKET_Y: usize = 2;
const PACKET_WIDTH_OR_FONT_SIZE: usize = 3;
const PACKET_HEIGHT: usize = 4; // Text: unused
const PACKET_R: usize = 5; // Image: unused
const PACKET_G: usize = 6; // Image: unused
const PACKET_B: usize = 7; // Image: unused
const PACKET_STRING_OFFSET: usize = 8; // Rectangle: unused
const PACKET_STRING_LENGTH: usize = 9; // Rectangle: unused

const PACKET_RECTANGLE_TYPE: f32 = 0.0;
const PACKET_TEXT_TYPE: f32 = 1.0;
const PACKET_IMAGE_TYPE: f32 = 2.0;

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
        let mut numeric_buffer: Vec<f32> = vec![0.0; draw_structures.len() * PACKET_SIZE];
        let mut string_table: Vec<u8> = Vec::new();

        for (index, draw_structure) in draw_structures.iter().enumerate() {
            let row_start: usize = index * PACKET_SIZE;
            let row: &mut [f32] = &mut numeric_buffer[row_start..row_start + PACKET_SIZE];

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
        row[PACKET_TYPE] = PACKET_RECTANGLE_TYPE;
        row[PACKET_X] = rectangle.location.x;
        row[PACKET_Y] = rectangle.location.y;
        row[PACKET_WIDTH_OR_FONT_SIZE] = rectangle.width;
        row[PACKET_HEIGHT] = rectangle.height;
        row[PACKET_R] = rectangle.color.r as f32;
        row[PACKET_G] = rectangle.color.g as f32;
        row[PACKET_B] = rectangle.color.b as f32;
        // PACKET_STRING_OFFSET / PACKET_STRING_LENGTH left as 0.0 (unused for Rectangle).
    }

    fn pack_text(text: &Text, row: &mut [f32], string_table: &mut Vec<u8>) {
        let (offset, length): (usize, usize) = Self::push_string(string_table, &text.content);

        row[PACKET_TYPE] = PACKET_TEXT_TYPE;
        row[PACKET_X] = text.location.x;
        row[PACKET_Y] = text.location.y;
        row[PACKET_WIDTH_OR_FONT_SIZE] = text.font_size;
        // PACKET_HEIGHT left as 0.0 (unused for Text).
        row[PACKET_R] = text.color.r as f32;
        row[PACKET_G] = text.color.g as f32;
        row[PACKET_B] = text.color.b as f32;
        row[PACKET_STRING_OFFSET] = offset as f32;
        row[PACKET_STRING_LENGTH] = length as f32;
    }

    fn pack_image(image: &Image, row: &mut [f32], string_table: &mut Vec<u8>) {
        let (offset, length): (usize, usize) = Self::push_string(string_table, &image.source);

        row[PACKET_TYPE] = PACKET_IMAGE_TYPE;
        row[PACKET_X] = image.location.x;
        row[PACKET_Y] = image.location.y;
        row[PACKET_WIDTH_OR_FONT_SIZE] = image.width;
        row[PACKET_HEIGHT] = image.height;
        // PACKET_R / PACKET_G / PACKET_B left as 0.0 (unused for Image).
        row[PACKET_STRING_OFFSET] = offset as f32;
        row[PACKET_STRING_LENGTH] = length as f32;
    }

    fn push_string(string_table: &mut Vec<u8>, value: &str) -> (usize, usize) {
        let offset: usize = string_table.len();
        let bytes: &[u8] = value.as_bytes();

        string_table.extend_from_slice(bytes);

        (offset, bytes.len())
    }
}