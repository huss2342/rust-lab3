use std::sync::atomic::AtomicBool;

// pub type LineNum = usize;
// pub type CharName = String;
// pub type Line = String;
// pub type LineTuple = (LineNum, CharName, Line);

// Command line argument position constants
pub static SCRIPT_NAME_POS: usize = 0;
pub static NETWORK_ADDR_POS: usize = 1;
pub static NUM_ARGS: usize = 2;

// pub static SCENE_TOKEN_INDEX: usize = 0;
// pub static SCENE_TITLE_INDEX: usize = 1;
// pub static CONFIG_FILE_INDEX: usize = 0;
// pub static SCRIPT_CONFIG_LINE_TOKENS: usize = 1;
//
// pub static FIRST_SCENE_FRAGMENT: usize = 0;

// Return value constants
pub static SUCCESS: u8 = 0;
pub static BAD_CMD_LINE: u8 = 1;
pub static FAILED_TO_GENERATE_SCRIPT: u8 = 2;

pub static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);


pub static TEMP_ERR_RETURN: u8 = 1;