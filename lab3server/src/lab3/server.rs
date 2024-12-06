/*
This server file defines the Server struct and its associated methods to manage TCP server operations.
It includes functionality to open a listener on a specified address, accept client connections, process
file requests, and handle file transmission while respecting cancellation flags and error conditions.
 */

use crate::lab3;
use lab3::server_declarations::*;
use std::sync::atomic::Ordering::SeqCst;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::thread::spawn;

pub struct Server {
    listener: Option<TcpListener>,
    listening_addr: String,
}

impl Server {

    pub fn new() -> Server {
        Server {
            listener: None,
            listening_addr: String::new(),
        }
    }

    /// Return false if the listener optional is none, otherwise return true
    fn is_open(&self) -> bool {
        self.listener.is_some()
    }


    /// open a TcpListener using the given address and store it in the struct
    pub fn open(&mut self, addr: &str) -> Result<(), u8> {
        match TcpListener::bind(addr) {
            Ok(listener) => {
                self.listener = Some(listener);
                self.listening_addr = addr.to_string();
                println!("Server listening on: {}", addr);
                Ok(())
            },
            Err(..) => {
                Err(TEMP_ERR_RETURN)
            }
        }
    }

    ///
    pub fn run(&mut self) -> Result<(), u8> {
        loop {
            if CANCEL_FLAG.load(SeqCst) || !self.is_open() {
                return Err(TEMP_ERR_RETURN);
            }

            let connection = self.listener.as_mut().unwrap().accept();
            if CANCEL_FLAG.load(SeqCst) {
                return Err(TEMP_ERR_RETURN);
            }
            match connection {
                Ok((mut stream, _addr)) => {
                    spawn(move || {
                        let mut file_name: String = String::new();
                        stream.read_to_string(&mut file_name).expect("Failed to Read String.");

                        // if file contains illegal file_name characters, return an appropriate error
                        if file_name.contains(|c: char| { c == '/' || c == '\\' || c == '$' }) {
                            writeln!(std::io::stderr().lock(), "ERROR: Cannot open file in another directory.").expect("Failed to write to stderr");
                            return Err(TEMP_ERR_RETURN);
                        }
                        // if file_name is quit, set cancel flag and return immediately
                        if file_name == "quit" {
                            CANCEL_FLAG.store(true, SeqCst);
                            return Ok(());
                        }

                        let mut file = match File::open(&file_name) {
                            Ok(file) => file,
                            Err(e) => {
                                stream.shutdown(Shutdown::Both).expect("Failed to shutdown connection.");
                                writeln!(std::io::stderr().lock(), "ERROR: Failed to open file '{}': {}", &file_name, e).expect("Failed to write to stderr");
                                return Err(TEMP_ERR_RETURN);
                            }
                        };
                        println!("sending file: {}", &file_name);
                        let mut buffer = [0; 240];  // TODO change 240 to a static non-temp value
                        loop {
                            match file.read(&mut buffer) {
                                Ok(0) => break,  // EOF reached, exit loop
                                Ok(n) => stream.write_all(&buffer[..n]).expect("Failed to write to stream"),
                                Err(e) => panic!("Failed to read from file: {}", e)
                            }
                        }
                        stream.shutdown(Shutdown::Write).expect("could not shutdown");
                        return Ok(());
                    }); // .join().expect("Failed on joining child thread.").expect("Failed on joining child thread 2."); FIXME remove
                },
                Err(..) =>  {
                    return Err(TEMP_ERR_RETURN); // failed to open connection
                }
            }
        }
    }
}