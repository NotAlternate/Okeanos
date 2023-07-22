use std::{process::{Command, Stdio}, io::{Stdout, Read, Write}, path::Path, fs::File, env, collections::HashMap};
use termion::raw::RawTerminal;
use crate::{prompt_handler, strings};

#[allow(deprecated)]
pub fn run_command(text: String, args: Vec<&str>, stdout: &mut RawTerminal<Stdout>, redirect: String) -> bool {
    let errors = strings::errors();

    stdout.suspend_raw_mode().unwrap();
    match args[0] {
        // Shell built-in commands
        "exit" => { println!("exit"); true },
        "cd" => { let where_to: String; match args.len() {
                1 => { where_to = env::home_dir().unwrap().to_string_lossy().to_string() }
                2 => { where_to = match Path::new(args[1]) {
                    exists if exists.exists() => {
                        if exists.is_file() { println!("{}", errors["cdArgumentIsNotADirectory"]); return false }
                        else { args[1].to_string() }
                    }, _ => { println!("{}", errors["pathDoesntExist"]); return false }
                }},
                _ => { println!("{}", errors["requiredSingleArgument"]); return false }
            } match env::set_current_dir(Path::new(&where_to)) {
                Ok(_) => (), Err(e) => println!("{} :: {}", errors["directorySetFail"], e)
        } false }

        // cd minus the cd but with run feature
        cd if Path::new(cd).exists() => {
            if Path::new(cd).is_file() { return process_create(cd, args, text, stdout, String::new(), errors, true) }
            else { match env::set_current_dir(cd) {
                Ok(_) => (), Err(e) => println!("{} :: {}", errors["directorySetFail"], e)
        }} false }

command => { process_create(command, args, text, stdout, redirect, errors, false) }}}

// fauncteions
fn process_create(command: &str, args: Vec<&str>, text: String, stdout: &RawTerminal<Stdout>, redirect: String, errors: HashMap<&'static str, String>,explicit: bool) -> bool {
    let empty = Command::new("/usr/bin/test").spawn().unwrap(); // idk
    let mut not_found: bool = false;

    let command_proc = match Command::new( if explicit { command.to_string() } else {
        // replace this with path crap later
        "/usr/bin/".to_string()+command
    }).args(&args[1..]).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() { Ok(t) => { t }, Err(_) => { not_found = true; empty } };

    if boolify(command_proc.id()) | not_found { print!("{}", prompt_handler::on_error(stdout, text)); if boolify(command_proc.id()) { return false }}
    if not_found { println!("\x1b[1;31mxx\x1b[0m Command `{}` does not exist", command); return false }

    let mut sadout = Vec::new(); command_proc.stdout.unwrap().read_to_end(&mut sadout).unwrap();
    let mut sadout = String::from_utf8_lossy(&sadout).to_string();
    let mut stderr = Vec::new(); command_proc.stderr.unwrap().read_to_end(&mut stderr).unwrap();
    let mut stderr = String::from_utf8_lossy(&stderr).to_string();

    // todo: make this better
    sadout = if sadout.ends_with('\n') || sadout.is_empty() { sadout } else { sadout+"\n" };
    stderr = if stderr.ends_with('\n') || stderr.is_empty() { stderr } else { stderr+"\n" };

    if redirect.is_empty() {
        print!("{}\n{}", sadout, stderr); // breaks the order but it does it for now.
    } else { match File::create(Path::new(&redirect)) {
        Ok(mut file) => { match file.write_all(sadout.as_bytes()) {
            Ok(_) => (), Err(e) => { eprintln!("{} :: {}", errors["fileWriteFail"], e); return false }
        }}, Err(e) => { eprintln!("{} :: {}", errors["fileCreationFail"], e); return false }
    }}

    stdout.activate_raw_mode().unwrap();
false }

// lazy way to check if proccess id is *valid*
fn boolify(result: u32) -> bool { if result == 0 { true } else { false } }