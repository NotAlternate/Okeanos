use std::{ io::{ stdin, stdout, Write }, env, env::current_dir, process::exit };
use termion::{ event::Key, input::TermRead, raw::IntoRawMode };
use whoami;
mod parser;
mod strings;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() != 1 {
        let mut index: usize = 1;
        while index < args.len() {
            let value = args[index].to_lowercase();
            match value.as_str() {
                "--version" | "-v" => { println!("{}", strings::get_c(strings::Command::Version)); exit(1); },
                _ => { eprintln!("{}", strings::get_e(strings::Errors::UnknownParameterPassed)); },
            }
        index += 1 }
    }

    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut colour = "\x1b[1;34m";
    let mut sign = "$";
    if whoami::username() == "root" { colour = "\x1b[1;31m"; sign = "#"; }

    let prompt = colour.to_string()+whoami::username().as_str()+"\x1b[0m@"+colour+whoami::hostname().as_str()+"\x1b[0m: "+current_dir().unwrap().to_str().unwrap()+"\n\x1b[0G"+sign+" ";

    let mut exit: bool = false;
    let mut done: bool = false;

    loop {
        print!("{}", prompt);
        stdout.flush().unwrap();

        let mut text = String::new();
        let mut column: usize = 0;

        for c in stdin().keys() {
            match c.as_ref().unwrap() {
                Key::Ctrl('c') => { exit = true; break }, // temporary?

                Key::Left => {
                    if column != 0 {
                        print!("\x1b[1D"); stdout.flush().unwrap();
                        column -= 1;
                    }
                }
                Key::Right => {
                    if column != text.len() {
                        print!("\x1b[1C"); stdout.flush().unwrap();
                        column += 1;
                    }
                }

                Key::Char('\n') => { done = true; break },

                Key::Backspace => {
                    if text.len() != 0 {
                        if column == text.len() {
                            text = text[0..(text.len()-1)].to_string();
                        } else {
                            let t = text[0..column].to_string();
                            text = t[0..t.len()-1].to_string()+text[column..text.len()].to_string().as_str();
                        }
                        print!("\x1b[2K\x1b[0G{} {}\x1b[{}G", sign, text, column+2);
                        stdout.flush().unwrap();
                        column -= 1;
                    }
                },

                Key::Char(c) => {
                    if column == text.len() {
                        text += c.to_string().as_str();
                    } else {
                        text = text[0..column].to_string()+c.to_string().as_str()+&text[column..text.len()];
                    }
                    print!("\x1b[2K\x1b[0G{} {}\x1b[{}G", sign, text, column+4);
                    stdout.flush().unwrap();
                    column += 1;
                },

                _ => ()
            }
        }
        if exit { break }
        if done {
            print!("\n\x1b[0G"); stdout.flush().unwrap();
            if parser::parse(text, sign.to_string(), &mut stdout) { break }
        }
    }
}