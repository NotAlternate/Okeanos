use std::{process::Command, io::Stdout, path::Path, env};
use termion::raw::RawTerminal;
use crate::{prompt_handler, strings};

#[allow(deprecated)]
pub fn run_command(text: String, args: Vec<&str>, stdout: &mut RawTerminal<Stdout>) -> bool {
    let errors = strings::errors();

    stdout.suspend_raw_mode().unwrap();
    match args[0] {
        // Shell built-in commands
        "exit" => { print!("exit"); true },
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

        // cd minus the cd
        cd if Path::new(cd).exists() => {
            if Path::new(cd).is_file() { println!("{}", errors["cdArgumentIsNotADirectory"]); return false }
            else { match env::set_current_dir(Path::new(args[0])) {
                Ok(_) => (), Err(e) => println!("{} :: {}", errors["directorySetFail"], e)
        }} false }

        // END OF BUILT-IN COMMANDS

        // Creating a new child process
        command => {
            let empty = Command::new("/usr/bin/test").spawn().unwrap(); // idk
            let mut not_found: bool = false;

            let mut command_proc = match Command::new("/usr/bin/".to_string()+command).args(&args[1..]).spawn() {
                Ok(t) => { t }, Err(_) => { not_found = true; empty }
            };

            if boolify(command_proc.id()) | not_found { print!("{}", prompt_handler::on_error(stdout, text)); }
            if not_found { println!("\x1b[1;31mxx\x1b[0m Command `{}` does not exist", command); }

            if command_proc.stdout.is_some() { print!("{:?}", command_proc.stdout); }
            if command_proc.stderr.is_some() { print!("{:?}", command_proc.stderr); }

            if command_proc.wait().unwrap().success() { stdout.activate_raw_mode().unwrap(); }
            else { stdout.activate_raw_mode().unwrap(); }
        false }
    }
}
// lazy way to check if proccess id is *valid*
fn boolify(result: u32) -> bool { if result == 0 { true } else { false } }