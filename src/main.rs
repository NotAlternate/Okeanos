use std::{ io::{ stdin, stdout, Write }, env, process::exit };
use termion::{ event::Key, input::TermRead, raw::IntoRawMode };
// mod config_handler;
mod prompt_handler;
mod strings;
mod utility;
mod parser;

fn main() {
    let errors = strings::errors(); let commands = strings::commands();

    // Termion does not support Windows, so Okeanos does not support Windows as well, if it wasn't clear to you.
    if utility::is_windows() { eprintln!("{}", errors["noNTSupport"]); exit(-1); }

    // Command line parameter parsing stuff
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() != 1 { for parameter in &args { match parameter.to_lowercase().as_str() {
        "--help" | "-h" => { println!("{}", commands["help"]); exit(0) }
        "--version" | "-v" => { println!("{}", commands["version"]); exit(0); },
        asdfghjkl => { if asdfghjkl == args[0] { () } else {
            eprintln!("{}", errors["unknownArgument"]);
        }},
    }}}


    // Shell starts here
    loop {
        let mut stdout = stdout().into_raw_mode().unwrap();

        print!("{}", prompt_handler::get_prompt(&stdout)); stdout.flush().unwrap();
        let mut text = String::new();
        let mut column: usize = 0;
        let mut exit = false;
        let mut done = false;

        for c in stdin().keys() { let prompt = prompt_handler::get_last_of_prompt(&stdout); match c.as_ref().unwrap() {
            Key::Ctrl('c') => { exit = true; break }, // temporary?
            Key::Char('\n') => { done = true; break },

            Key::Left => { if column != 0 { print!("\x1b[1D"); stdout.flush().unwrap(); column -= 1; }}
            Key::Right => { if column != text.len() { print!("\x1b[1C"); stdout.flush().unwrap(); column += 1; }}

            Key::Backspace => { if text.len() != 0 {
                if column == text.len() { text = text[0..(text.len()-1)].to_string(); }
                else { let t = text[0..column].to_string();
                       text = t[0..t.len()-1].to_string()+text[column..text.len()].to_string().as_str(); }
                column -= 1;
                print!("\x1b[2K\x1b[0G{}{}\x1b[{}G", prompt, text, column+prompt.len()+1); stdout.flush().unwrap();
            }},

            Key::Char(c) => {
                if column == text.len() { text += c.to_string().as_str(); }
                else { text = text[0..column].to_string()+c.to_string().as_str()+&text[column..text.len()]; }
                column += 1;
                print!("\x1b[2K\x1b[0G{}{}\x1b[{}G", prompt, text, column+prompt.len()+1); stdout.flush().unwrap();
            },

            _ => ()
        }}
        if exit { break }
        if done { print!("\n\x1b[0G"); stdout.flush().unwrap();
            if parser::parse(text, prompt_handler::get_last_of_prompt(&stdout), &mut stdout) { break }
        }
    }
}