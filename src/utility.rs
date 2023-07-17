use std::{process::{Command, exit}, str, io::{Stdout, stdout, stdin, BufRead, Write}, env, path::PathBuf};
use termion::raw::RawTerminal;
use crate::strings;
use regex::Regex;

// welcome to the code that holds terrible code.

#[cfg(target_os = "windows")]
pub fn is_windows() -> bool { true }
#[cfg(not(target_os = "windows"))]
pub fn is_windows() -> bool { false }

pub fn strip_ansi(text: &str, full: bool) -> String { Regex::new(if full { r"\x1B\[([0-9]{1,2}(;[0-9]{1,2})?)?[mGK]" } else { r"\x1B\[([0-9]{1,2})?[GK]" }).unwrap().replace_all(text, "").to_string() }

// really terrible way to get git info
// but if it works, it works.

// prompt_handler.rs
fn exit_stdterm(stdout: &RawTerminal<Stdout>, code: i32) { // change into standard term, exit.
    stdout.suspend_raw_mode().unwrap();
    exit(code);
}

pub fn get_git_info(stdout: &RawTerminal<Stdout>,colour: &String) -> String {
    let errors = strings::errors();
    let mut in_a_repo = true;

    let command_repo = Command::new("/usr/bin/".to_string()+"git").args(["config", "--get", "remote.origin.url"]).output().expect("");
    let command_branch = Command::new("/usr/bin/".to_string()+"git").args(["branch", "--show-current"]).output().expect("");

    let repo = match str::from_utf8(&command_repo.stdout) {
        Ok(val) => { match val.split('/').last() {
            Some(val) => val.split(".git").next().unwrap(),
            _ => { eprintln!("{}", errors["invalidRepoUrl"]); exit_stdterm(stdout, -1); "" }
        }},
        Err(_) => { eprintln!("{}", errors["utf8Conversion"]); exit_stdterm(stdout, -1); "" },
    };

    if !command_branch.stderr.is_empty() { in_a_repo = false; }
    let branch = match str::from_utf8(&command_branch.stdout) {
        Ok(val) => { match val {
            "" => { in_a_repo = false; "" }
            val => val.strip_suffix('\n').unwrap(),
        }},
        Err(_) => { eprintln!("{}", errors["utf8Conversion"]); exit_stdterm(stdout, -1); "" },
    };

    if in_a_repo { format!("{}[\x1b[0m{}{}:\x1b[0m{}{}]\x1b[0m", colour, repo, colour, branch, colour) }
    else { "".to_string() }
}


#[allow(deprecated)]
pub fn shorten_path(current_path: PathBuf) -> String {
    let home_dir = env::home_dir().unwrap();
    if current_path.starts_with(&home_dir) {
        let mut relative_path = PathBuf::from("~");
        relative_path.push(current_path.strip_prefix(&home_dir).unwrap());
        relative_path.to_string_lossy().to_string()
    } else { current_path.to_string_lossy().to_string() }
}


// config_handler.rs
pub fn prompt_input(text: &String) -> bool {
    loop {
        print!("{} :: [Y/n]: ", text); stdout().flush().unwrap();
        match stdin().lock().lines().next().unwrap() {
            Ok(content) => { match content.as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                "" => return true,
                _ => { eprintln!("{}", strings::warnings()["invalidChoice"]); continue; },
            }},
            Err(error) => { eprintln!("{} :: {}", strings::errors()["stdinFail"], error); exit(-1); },
        }
    }
}

pub fn prompt_wait(text: &String) {
    print!("{}", text); stdout().flush().unwrap();
    match stdin().lock().lines().next().unwrap() {
        Ok(content) => { match content.as_str() { _ => (), }},
        Err(error) => { eprintln!("{} :: {}", strings::errors()["stdinFail"], error); exit(-1); },
    }
}

pub fn clear_screen() { print!("\x1B[2J\x1B[3J\x1B[H"); }