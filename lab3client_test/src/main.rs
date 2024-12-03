use std::env;
use std::io::{stdout, Read, Write};
use std::net::{Shutdown, TcpStream};
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
            stream.shutdown(Shutdown::Write).expect("could not shutdown");
            if token != "quit" {
                loop {
                    let mut text: String = String::new();
                    let read_ret = stream.read_to_string(&mut text).expect("Reading to String Failed.");
                    if read_ret == 0 {
                        break;
                    }
                    writeln!(stdout(), "WRITING: ").expect("Failed to write to stdout."); // TODO Idk if we want this or not, examine later
                    println!("{}", &text);
                }
                Ok(())
            } else {
                let time: Duration = Duration::new(1, 0);
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