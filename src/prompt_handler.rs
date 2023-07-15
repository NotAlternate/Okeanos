use std::{io::Stdout, env, process::exit};
use termion::raw::RawTerminal;
use crate::{utility, strings};
use whoami;


struct DefaultPrompt<'a> {
    accent: String,
    sign: &'a str
} impl DefaultPrompt<'_> {
    pub fn new() -> Self {
        let mut accent = "\x1b[1;34m".to_string();
        let sign = match whoami::username().as_str() {
            "root" => { accent = "\x1b[1;31m".to_string(); "#" },
            _ => "$"
        }; Self { accent, sign }
    }
    pub fn get(self, stdout: &RawTerminal<Stdout>) -> String {
        let git_info = utility::get_git_info(&stdout, &self.accent);
        let current_path = match env::current_dir() {
            Ok(a) => utility::shorten_path(a),
            Err(e) => { eprintln!("{} :: {}", strings::errors()["pathRetrivalFail"], e); exit(-1); }
        };
        format!("{}{}\x1b[0m@{}{}\x1b[0m {}: {}\n{} ",
            self.accent, whoami::username(), self.accent, whoami::hostname(), current_path, git_info, self.sign
        )
    }
    pub fn on_error(self, stdout: &RawTerminal<Stdout>, text: String) -> String {
        format!("\x1b[1F\x1b[2K{}\x1b[1;31m{}\x1b[0m {}\n",get_last_of_prompt(stdout, true).strip_suffix(&format!("{} ", self.sign)).unwrap().to_string(), self.sign, text)
    }
}


pub fn get_prompt(stdout: &RawTerminal<Stdout>) -> String {
    DefaultPrompt::new().get(stdout) // Temporary, custom prompts are planned.
}

pub fn on_error(stdout: &RawTerminal<Stdout>, text: String) -> String {
    DefaultPrompt::new().on_error(stdout, text) // Temporary, custom prompts are planned.
}


pub fn get_last_of_prompt(stdout: &RawTerminal<Stdout>, colours: bool) -> String { utility::strip_ansi(get_prompt(stdout).split("\n").last().unwrap(), if colours { false } else { true }) }
pub fn get_last_len(stdout: &RawTerminal<Stdout>) -> usize { get_last_of_prompt(stdout, false).len()+1 }