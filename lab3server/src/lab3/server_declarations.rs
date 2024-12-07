/*
This server file defines shared constants and global variables for the server application. It includes
command-line argument positions, return value codes, and a cancellation flag to manage server operations
and error handling.
 */
use std::sync::atomic::AtomicBool;

// Command line argument position constants
pub static SCRIPT_NAME_POS: usize = 0;
pub static NETWORK_ADDR_POS: usize = 1;
pub static NUM_ARGS: usize = 2;

// Return value constants
pub static SUCCESS: u8 = 0;
pub static BAD_CMD_LINE: u8 = 1;
pub static FAILED_TO_GENERATE_SCRIPT: u8 = 2;

pub static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

pub static CANNOT_RUN_SERVER: u8 = 1;
pub static FAILED_TO_OPEN_CONNECTION: u8 = 2;
pub static FAILED_TO_OPEN_FILE: u8 = 3;
pub static FAILED_TO_BIND: u8 = 4;

pub const BUFFER_SIZE: usize = 240;