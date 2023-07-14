use whoami;
use crate::utility;
use crate::strings;
use termion::raw::RawTerminal;
use std::io::Stdout;
use std::env;
use std::process::exit;

fn default_prompt(stdout: &RawTerminal<Stdout>) -> String {
    let mut colour = "\x1b[1;34m".to_string();
    let sign = match whoami::username().as_str() {
        "root" => { colour = "\x1b[1;31m".to_string(); "#" },
        _ => "$"
    };

    let git_info = utility::get_git_info(&stdout, &colour);
    let current_path = match env::current_dir() {
        Ok(a) => utility::shorten_path(a),
        Err(e) => { eprintln!("{} :: {}", strings::errors()["pathRetrivalFail"], e); exit(-1); }
    };
    format!("{}{}\x1b[0m@{}{}\x1b[0m {}: {}\n\x1b[0G{} ",
        colour, whoami::username(), colour, whoami::hostname(), current_path, git_info, sign
    )
}

pub fn get_prompt(stdout: &RawTerminal<Stdout>) -> String {
    default_prompt(stdout)
}

pub fn get_last_of_prompt(stdout: &RawTerminal<Stdout>) -> String {
    default_prompt(stdout).split("\n\x1b[0G").last().unwrap().to_string()
}