use std::{ io::{ stdin, stdout, Write }, env, process::exit };
use termion::{ event::Key, input::TermRead, raw::IntoRawMode };
mod strings;
mod parser;
mod utility;
use whoami;

fn main() {
    let errors = strings::errors(); let commands = strings::commands();

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
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut colour = "\x1b[1;34m".to_string();
    let sign = match whoami::username().as_str() {
        "root" => { colour = "\x1b[1;31m".to_string(); "#" },
        _ => "$"
    };

    loop {
        let git_info = utility::get_git_info(&stdout, &colour);
        let current_path = match env::current_dir() {
            Ok(a) => utility::shorten_path(a),
            Err(e) => { eprintln!("{} :: {}", errors["pathRetrivalFail"], e); exit(-1); }
        };
        let prompt = format!("{}{}\x1b[0m@{}{}\x1b[0m {}: {}\n\x1b[0G{} ",
            colour, whoami::username(), colour, whoami::hostname(), current_path, git_info, sign
        );
        
        print!("{}", prompt); stdout.flush().unwrap();
        let mut text = String::new();
        let mut column: usize = 0;
        let mut exit = false;
        let mut done = false;

        for c in stdin().keys() { match c.as_ref().unwrap() {
            Key::Ctrl('c') => { exit = true; break }, // temporary?
            Key::Char('\n') => { done = true; break },

            Key::Left => { if column != 0 { print!("\x1b[1D"); stdout.flush().unwrap(); column -= 1; }}
            Key::Right => { if column != text.len() { print!("\x1b[1C"); stdout.flush().unwrap(); column += 1; }}

            Key::Backspace => { if text.len() != 0 {
                if column == text.len() { text = text[0..(text.len()-1)].to_string(); }
                else { let t = text[0..column].to_string();
                       text = t[0..t.len()-1].to_string()+text[column..text.len()].to_string().as_str(); }
                print!("\x1b[2K\x1b[0G{} {}\x1b[{}G", sign, text, column+2); stdout.flush().unwrap();
            column -= 1; }},

            Key::Char(c) => {
                if column == text.len() { text += c.to_string().as_str(); }
                else { text = text[0..column].to_string()+c.to_string().as_str()+&text[column..text.len()]; }
                print!("\x1b[2K\x1b[0G{} {}\x1b[{}G", sign, text, column+4); stdout.flush().unwrap();
            column += 1; },

            _ => ()
        }}
        if exit { break }
        if done { print!("\n\x1b[0G"); stdout.flush().unwrap();
            if parser::parse(text, sign.to_string(), &mut stdout) { break }
        }
    }
}