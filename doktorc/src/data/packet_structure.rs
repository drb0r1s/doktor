pub const PACKET_SIZE: usize = 16;

pub const PACKET_TYPE: usize = 0;
pub const PACKET_X: usize = 1;
pub const PACKET_Y: usize = 2;
pub const PACKET_WIDTH: usize = 3;
pub const PACKET_HEIGHT: usize = 4;
pub const PACKET_BACKGROUND_COLOR: usize = 5;
pub const PACKET_STRING_OFFSET: usize = 6;
pub const PACKET_STRING_LENGTH: usize = 7;
pub const PACKET_CONTENT_COLOR: usize = 8;
pub const PACKET_CONTENT_SIZE: usize = 9;
pub const PACKET_CONTENT_FONT: usize = 10;

pub const PACKET_RECTANGLE_TYPE: f32 = 0.0;
pub const PACKET_TEXT_TYPE: f32 = 1.0;
pub const PACKET_IMAGE_TYPE: f32 = 2.0;