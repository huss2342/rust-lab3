pub mod lab3;

use std::env;
use std::io::Write;
use lab3::server_declarations::*;
use lab3::return_wrapper::ReturnWrapper;
use lab3::server::Server;

fn main() -> ReturnWrapper {
    let args: Vec<String> = env::args().collect();
    if args.len() != NUM_ARGS {
        usage(&args[SCRIPT_NAME_POS]);
        return ReturnWrapper::new(Err(BAD_CMD_LINE));
    }

    let mut server = Server::new();
    match server.open(&args[NETWORK_ADDR_POS]) {
        Ok(()) => ReturnWrapper::new(server.run()),
        Err(_e) => ReturnWrapper::new(Err(4))
    }
}

// prints helpful usage message
fn usage(script_name: &String) {
    writeln!(std::io::stderr().lock(), "usage: {} <network_address>", script_name).expect("Failed to write to stderr");
}