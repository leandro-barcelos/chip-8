pub const FRAME_TIME_60HZ: f32 = 1.0 / 60.0;
pub const FRAME_TIME_700HZ: f32 = 1.0 / 700.0;
pub const PROGRAM_START_ADDRESS: usize = 0x200;
pub const FONT_START_ADRESS: usize = 0x50;

// Display
pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const SCALE: usize = 10;

// Quirks
pub const VF_RESET: bool = true;
pub const SHIFT_USE_VY: bool = false;
pub const BNNN: bool = false;
pub const STORE_LOAD_INCREMENTS_I: bool = true;
pub const DISPLAY_WAIT: bool = true;
