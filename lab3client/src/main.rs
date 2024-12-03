pub mod lab3;
use std::io::Write;
use std::env;
use std::sync::atomic::Ordering;
use lab3::declarations::*;
use lab3::play::Play;
use lab3::return_wrapper::ReturnWrapper;


fn main() -> ReturnWrapper {
    // open script config file
    let mut script_file_name = String::new();

    match parse_args(&mut script_file_name) {
        Ok(()) => {
            let mut play = Play::new();
            match play.prepare(&script_file_name) {
                Ok(()) => {
                    play.recite();
                },
                Err(..) => return ReturnWrapper::new(Err(FAILED_TO_GENERATE_SCRIPT)),
            }
        },
        Err(..) => {
            writeln!(std::io::stderr().lock(), "ERROR: Bad command line arguments provided.").expect("Failed to write to stderr");
            return ReturnWrapper::new(Err(BAD_CMD_LINE))
        }
    }
    ReturnWrapper::new(Ok(()))
}

fn parse_args(script_file_name: &mut String) -> Result<(), u8> {

    let args: Vec<String> = env::args().collect();

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS ||
        (args.len() == MAX_ARGS && args[OPT_WHINGE_POS] != "whinge") {
        usage(&args[SCRIPT_NAME_POS]);
        return Err(BAD_CMD_LINE);
    }

    *script_file_name = args[SCRIPT_FILE_POS].clone();

    if args.len() == MAX_ARGS {
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }
    Ok(())
}

// prints helpful usage message
fn usage(script_name: &String) {
    writeln!(std::io::stderr().lock(), "usage: {} <script_file_name> [whinge]", script_name).expect("Failed to write to stderr");
}
