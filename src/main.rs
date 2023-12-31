use termion::{ event::Key, input::TermRead, raw::IntoRawMode };
use std::{ io::{ stdin, stdout, Write }, env, process::exit };
use okeanos::*;

fn main() {
    // Termion does not support Windows, so Okeanos does not support Windows as well, if it wasn't clear to you.
    if utility::is_windows() { eprintln!("{}", strings::fetch("errors.noNTSupport")); exit(-1); }

    // Command line parameter parsing stuff
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() != 1 { for parameter in &args { match parameter.to_lowercase().as_str() {
        "--help" | "-h" => { println!("{}", strings::fetch("commands.help")); exit(0) }
        "--version" | "-v" => { println!("{}", strings::fetch("commands.version")); println!("Checking for updates.."); version::update("https://raw.githubusercontent.com/NotAlternate/Okeanos/main/VERSION"); exit(0); },
        "--update" | "-u" => { version::update("https://raw.githubusercontent.com/NotAlternate/Okeanos/main/VERSION"); exit(0); },
        asdfghjkl => { if asdfghjkl == args[0] { () } else {
            eprintln!("{}", strings::fetch("errors.unknownArgument"));
        }},
    }}}

    // Config stuff
    let mut config = config_handler::Configurations::new();
    // println!("{:?}", config.data); // debug

    // Shell starts here
    loop {
        let mut stdout = stdout().into_raw_mode().unwrap(); stdout.suspend_raw_mode().unwrap();

        print!("{}", prompt_handler::get_prompt(&stdout)); stdout.flush().unwrap();
        let mut text = String::new();
        let mut cursor: usize = 0;
        let mut done = false;

        stdout.activate_raw_mode().unwrap();

        for c in stdin().keys() {
            let prompt = prompt_handler::get_last_of_prompt(&stdout, true);
            let length = prompt_handler::get_last_len(&stdout);
            match match c.as_ref() { Ok(c) => c, Err(e) => {
                stdout.suspend_raw_mode().unwrap(); print!("\x1b[2J");
                utility::prompt_wait(&format!("{} :: {}\nExitting on keypress..", strings::fetch("errors.unwrapFail"), e)); exit(-1);
            }} {
                Key::Ctrl('c') => { done=true; text="".to_string(); break },
                Key::Char('\n') => { done=true; break },

                
                // configuration, history and stuff
                Key::Up => { println!("\x1b[1E{}\x1b[2F\x1b[{}G", "history not implemented", length+cursor); }
                Key::Down => { println!("\x1b[1E{}\x1b[2F\x1b[{}G", "history not implemented", length+cursor); }


                Key::Left => { if cursor != 0 { print!("\x1b[1D"); stdout.flush().unwrap(); cursor -= 1; }}
                Key::Right => { if cursor != text.len() { print!("\x1b[1C"); stdout.flush().unwrap(); cursor += 1; }}

                Key::Backspace => { if text.len() != 0 && cursor != 0 {
                    if cursor == text.len() { text = text[0..(text.len()-1)].to_string(); }
                    else { let t = text[0..cursor].to_string();
                           text = t[0..t.len()-1].to_string()+text[cursor..text.len()].to_string().as_str(); }
                    cursor -= 1;
                    print!("\x1b[2K\x1b[0G{}{}\x1b[{}G", prompt, text, length+cursor); stdout.flush().unwrap();
                }},

                Key::Char(c) => {
                    if cursor == text.len() { text += c.to_string().as_str(); }
                    else { text = text[0..cursor].to_string()+c.to_string().as_str()+&text[cursor..text.len()]; }
                    cursor += 1;
                    print!("\x1b[2K\x1b[0G{}{}\x1b[{}G", prompt, text, length+cursor); stdout.flush().unwrap();
                },

                _ => ()
        }}
        if done {
            stdout.suspend_raw_mode().unwrap(); println!();
            if parser::parse(text, &mut stdout) { break }
            config.check(); // Checking for configuration's existence.
            stdout.activate_raw_mode().unwrap();
        }
    }
}