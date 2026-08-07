pub const PACKET_SIZE: usize = 41;

pub const PACKET_TYPE: usize = 0;

pub const PACKET_ID: usize = 1;

pub const PACKET_X: usize = 2;
pub const PACKET_Y: usize = 3;

pub const PACKET_WIDTH: usize = 4;
pub const PACKET_HEIGHT: usize = 5;

pub const PACKET_IS_SCROLLABLE_X: usize = 6;
pub const PACKET_IS_SCROLLABLE_Y: usize = 7;

pub const PACKET_SCROLLABLE_WIDTH: usize = 8;
pub const PACKET_SCROLLABLE_HEIGHT: usize = 9;

pub const PACKET_SCROLL_OFFSET_X: usize = 10;
pub const PACKET_SCROLL_OFFSET_Y: usize = 11;

pub const PACKET_CLIP_X_START: usize = 12;
pub const PACKET_CLIP_X_END: usize = 13;
pub const PACKET_CLIP_Y_START: usize = 14;
pub const PACKET_CLIP_Y_END: usize = 15;

pub const PACKET_BACKGROUND_COLOR: usize = 16;
pub const PACKET_BACKGROUND_COLOR_ALPHA: usize = 17;

pub const PACKET_STRING_OFFSET: usize = 18;
pub const PACKET_STRING_LENGTH: usize = 19;

pub const PACKET_CONTENT_COLOR: usize = 20;
pub const PACKET_CONTENT_COLOR_ALPHA: usize = 21;
pub const PACKET_CONTENT_SIZE: usize = 22;
pub const PACKET_CONTENT_FONT: usize = 23;

pub const PACKET_BORDER_TOP_COLOR: usize = 24;
pub const PACKET_BORDER_TOP_COLOR_ALPHA: usize = 25;
pub const PACKET_BORDER_TOP_SIZE: usize = 26;
pub const PACKET_BORDER_TOP_TYPE: usize = 27;

pub const PACKET_BORDER_BOTTOM_COLOR: usize = 28;
pub const PACKET_BORDER_BOTTOM_COLOR_ALPHA: usize = 29;
pub const PACKET_BORDER_BOTTOM_SIZE: usize = 30;
pub const PACKET_BORDER_BOTTOM_TYPE: usize = 31;

pub const PACKET_BORDER_LEFT_COLOR: usize = 32;
pub const PACKET_BORDER_LEFT_COLOR_ALPHA: usize = 33;
pub const PACKET_BORDER_LEFT_SIZE: usize = 34;
pub const PACKET_BORDER_LEFT_TYPE: usize = 35;

pub const PACKET_BORDER_RIGHT_COLOR: usize = 36;
pub const PACKET_BORDER_RIGHT_COLOR_ALPHA: usize = 37;
pub const PACKET_BORDER_RIGHT_SIZE: usize = 38;
pub const PACKET_BORDER_RIGHT_TYPE: usize = 39;

pub const PACKET_OPACITY: usize = 40;

pub const PACKET_RECTANGLE_TYPE: f32 = 0.0;
pub const PACKET_TEXT_TYPE: f32 = 1.0;
pub const PACKET_IMAGE_TYPE: f32 = 2.0;