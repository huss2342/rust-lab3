use std::env;
use std::io::{stdout, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

static SCRIPT_NAME_POS: usize = 0;
static NETWORK_ADDR: usize = 1;
static SERVER_TOKEN: usize = 2;
static NUM_ARGS: usize = 3;

static BAD_CMD_LINE: u8 = 1;
static BAD_CONNECTION: u8 = 2;



fn main() -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    if args.len() != NUM_ARGS {
        usage(&args[SCRIPT_NAME_POS]);
        return Err(BAD_CMD_LINE);
    }
    let addr: &String = &args[NETWORK_ADDR];
    let token: &String = &args[SERVER_TOKEN];

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            let token_buf = token.as_bytes();
            stream.write_all(token_buf).expect("Failed to Write to Stream.");
            if token != "quit" {
                let mut text = String::new();
                while stream.read_to_string(&mut text).unwrap() != 0 {
                    writeln!(stdout(), "WRITING").expect("Failed to write to stdout.");
                    println!("WRITING WITH PRINTLN");
                    writeln!(stdout(), "{}", &text).expect("Failed to write to stdout.");
                    text = String::new();
                }
                Ok(())
            } else {
                let time: Duration = Duration::new(1, 0);
                println!("going to sleep");
                std::thread::sleep(time);
                TcpStream::connect(addr).expect("Failed to Connect for Shutdown.");
                Ok(())
            }
        },
        Err(..) => Err(BAD_CONNECTION)
    }
}


// prints helpful usage message
fn usage(script_name: &String) {
    writeln!(std::io::stderr().lock(), "usage: {} <network_address> <token>", script_name).expect("Failed to write to stderr");
}