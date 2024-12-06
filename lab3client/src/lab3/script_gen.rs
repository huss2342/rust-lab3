/*
This client file provides utilities for reading and processing text from files or network streams to
support script generation. It includes functionality to handle different input sources, trim and store
file lines, and facilitate buffered reading of character configurations or scripts.
 */

use std::io::{Read, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{Shutdown, TcpStream};
use regex::Regex;
use crate::lab3::declarations::*;

pub type CharacterTextFile = String;

// line numbers in character config files
pub static TITLE_LINE: usize = 0;
pub static CHARACTER_CONFIG_LINE: usize = 1;

// token indices and number of tokens
pub static CHARACTER_NAME_CONFIG_LINE_INDEX: usize = 0;
pub static CHARACTER_FILE_CONFIG_LINE_INDEX: usize = 1;
pub static CONFIG_LINE_TOKENS: usize = 2;


pub enum BufferedReaderTypes {
    Stream(TcpStream),
    File(File),
}

impl Read for BufferedReaderTypes {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            BufferedReaderTypes::File(file) => {
                file.read(buf)
            },
            BufferedReaderTypes::Stream(stream) => {
                stream.read(buf)
            }
        }
    }
}

// documentation example taken from:
// https://deterministic.space/machine-readable-inline-markdown-code-cocumentation.html#markdown-formatting-conventions

///
/// Fills a vector with the lines in a file with the given name.
///
/// # Parameters
///
/// - `file_name`: A reference to a String that holds the name of the file to read
/// - `file_lines`: A mutable reference to a vector where the lines will be added
///
/// # Returns
///
/// A `Result` which is:
///
/// - `Ok`: The file was successfully read and inputted into `file_lines`
/// - `Err`: A `u8` value representing an error: unable to generate the script.
///     This can occur if the file could not be opened or the function `read_line`
///     returned an error while trying to read a line from the file.
///
pub fn grab_trimmed_file_lines(file_name: &String, file_lines: &mut Vec<String>) -> Result<(), u8> {
    // https://doc.rust-lang.org/std/io/struct.BufReader.html
    let mut reader = match get_buffered_reader(file_name) {
        Ok(reader) => reader,
        Err(e) => {
            writeln!(std::io::stderr().lock(), "ERROR: Failed to open file '{}': {}", file_name, e).expect("Failed to write to stderr");
             return Err(FAILED_TO_GENERATE_SCRIPT);
        }
    };
    let buf_reader_type = reader.get_mut();

    // run code for if the buffered reader holds a File type
    match buf_reader_type {
        BufferedReaderTypes::File(_) | BufferedReaderTypes::Stream(_) => {
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => return Ok(()), // EOF reached
                    Ok(..) => file_lines.push(line.trim().to_string()),
                    Err(e) => {
                        writeln!(std::io::stderr().lock(), "ERROR: Failed to read line '{}': {}", file_name, e).expect("Failed to write to stderr");
                        return Err(FAILED_TO_GENERATE_SCRIPT);
                    }
                }
            }
        }
    }
}


pub fn get_buffered_reader(text: &String) -> Result<BufReader<BufferedReaderTypes>, u8> {
    // check whether the string that was passed in begins with 'net:'
    let pattern = r"^net:(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}):(\d{1,5}):(.+)$";
    let re = Regex::new(pattern).unwrap();
    if re.is_match(text) {
        let sub_strs: Vec<&str> = text.split(':').collect();
        let addr: String = format!("{}:{}", sub_strs[1], sub_strs[2]); // TODO get rid of these hardcodes
        let file_name: String = sub_strs[3].to_string();

        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                let file_bytes = file_name.as_bytes();
                if stream.write(file_bytes).is_err() {
                    return Err(TCP_CONNECTION_FAILED);
                }
                if stream.shutdown(Shutdown::Write).is_err() {
                    return Err(TCP_CONNECTION_FAILED);
                }
                Ok(BufReader::new(BufferedReaderTypes::Stream(stream)))
            },
            Err(..) => {
                Err(TCP_CONNECTION_FAILED)
            }
        }
    } else {
        // consider text to be a file name
        let file_res = File::open(text);
        match file_res {
            Ok(file) => Ok(BufReader::new(BufferedReaderTypes::File(file))),
            Err(..) => Err(FILE_OPEN_FAILED)
        }
    }
}


