use std::{process::{Command, exit}, str, io::Stdout, env, path::PathBuf};
use termion::raw::RawTerminal;
use okeanos::strings;

// really terrible way to get git info
// but if it works, it works.

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


#[allow(deprecated)] // Termion only works in linux so who cares about windows.
pub fn shorten_path(current_path: PathBuf) -> String {
    let home_dir = env::home_dir().unwrap();
    if current_path.starts_with(&home_dir) {
        let mut relative_path = PathBuf::from("~");
        relative_path.push(current_path.strip_prefix(&home_dir).unwrap());
        relative_path.to_string_lossy().to_string()
    } else { current_path.to_string_lossy().to_string() }
}