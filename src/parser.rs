use std::{process::Command, io::Stdout};
use termion::raw::RawTerminal;


pub fn parse(text: String, sign: String, stdout: &mut RawTerminal<Stdout>) -> bool { if text == "" { return false }
    let args = text.split_whitespace().collect::<Vec<&str>>();
    match args[0] {
        // Shell built-in commands
        "exit" => { print!("exit"); true },

        // Creating a new child process
        command => {
            let empty = Command::new("/usr/bin/test").spawn().unwrap(); // idk
            let mut not_found: bool = false;

            let mut command_proc = match Command::new("/usr/bin/".to_string()+command).args(&args[1..]).spawn() {
                Ok(t) => { t }, Err(_) => { not_found = true; empty }
            };

            if boolify(command_proc.id()) | not_found { print!("\x1b[1F\x1b[2K\x1b[1;31m{}\x1b[0m {}\n\x1b[0G", sign, text); }
            if not_found { print!("\x1b[1;31mx\x1b[0m Command `{}` does not exist\n\x1b[0G", command); }

            stdout.suspend_raw_mode().unwrap();

            if command_proc.stdout.is_some() { print!("{:?}", command_proc.stdout); }
            if command_proc.stderr.is_some() { print!("{:?}", command_proc.stderr); }

            if command_proc.wait().unwrap().success() { stdout.activate_raw_mode().unwrap(); }
            else { stdout.activate_raw_mode().unwrap(); }
        false }
    }
}
// lazy way to check if proccess id is *valid*
fn boolify(result: u32) -> bool { if result == 0 { true } else { false } }