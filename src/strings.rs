use std::collections::HashMap;

pub fn errors() -> HashMap<&'static str, String> { let mut map = HashMap::new(); for current in [
    // Command line arguments parser
    ("unknownArgument", "An unknown argument has been passed."),

    // get_git_info()
    ("utf8Conversion", "Unexpected error while attempting to convert stdout into &str"),
    ("invalidRepoUrl", "An invalid url was retrived.")
].iter().cloned().collect::<HashMap<&str, &str>>() { map.insert(current.0, format!("\x1b[31mxx\x1b[0m {}", current.1)); } map }

const VERSION: &str = "v0.1.0";
pub fn commands() -> HashMap<&'static str, String> { [
    ("help", format!("\x1b[1mOkeanos {}\x1b[0m\nA shell project done \x1b[4mterrible\x1b[0m.\n\nProgram parameters:\n    \x1b[1mOkeanos\x1b[0m <INPUT> [FLAGS]\n\nProgram flags:\n    \x1b[1m--help\x1b[0m & \x1b[1m-h\x1b[0m  ::  Shows Okeanos's description and usage.\n    \x1b[1m--version\x1b[0m & \x1b[1m-v\x1b[0m  ::  Outputs current version of the shell.", VERSION)),
    ("version", VERSION.to_string()),
].iter().cloned().collect::<HashMap<&str, String>>() }