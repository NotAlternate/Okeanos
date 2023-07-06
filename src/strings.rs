pub enum Errors {
    UnknownParameterPassed,
} pub fn get_e(code: Errors) -> String {
    let symbol: &str = "\x1b[31;1mxx\x1b[0m";
    match code {
        Errors::UnknownParameterPassed => format!("{} An unknown parameter has been passed.", symbol),
    }
}


pub enum Command {
    Version,
} pub fn get_c(code: Command) -> String {
    match code {
        Command::Version => "0.1.0".to_string(),
    }
}