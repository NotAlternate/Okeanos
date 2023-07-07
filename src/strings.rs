pub enum Errors {
    UnknownParameterPassed,
} pub fn get_e(code: Errors) -> String {
    let symbol: &str = "\x1b[31;1mxx\x1b[0m";
    match code {
        Errors::UnknownParameterPassed => format!("{} An unknown parameter has been passed.", symbol),
    }
}


pub enum Command {
    Help,
    Version,
} pub fn get_c(code: Command) -> String {
    match code {
        Command::Help => "\x1b[1mOkeanos ".to_string()+get_c(Command::Version).as_str()+"\x1b[0m\nA shell project done \x1b[4mterrible\x1b[0m.\n\nProgram parameters:\n    \x1b[1mOkeanos\x1b[0m <INPUT> [FLAGS]\n\nProgram flags:\n    \x1b[1m--help\x1b[0m & \x1b[1m-h\x1b[0m  ::  Shows Okeanos's description and usage.\n    \x1b[1m--version\x1b[0m & \x1b[1m-v\x1b[0m  ::  Outputs current version of the shell.",
        Command::Version => "v0.1.0".to_string(),
    }
}