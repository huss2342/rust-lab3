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

                        let mut file_text: &mut [u8] = &mut [0; 240]; // TODO change 240 to a static non-temp value
                        while file.read(&mut file_text).unwrap() != 0 {
                            stream.write_all(&mut file_text).expect("Failed to Write to Stream.");
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