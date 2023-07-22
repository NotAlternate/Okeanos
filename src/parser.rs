use std::{io::{Stdout, Write}, io::stdout, process::exit, env};
use okeanos::{commands, strings};
use termion::raw::RawTerminal;

pub fn parse(text: String, stdout: &mut RawTerminal<Stdout>) -> bool {
    // let errors = strings::errors();

    // Return here is for "should_i_exit"
    if text.replace(" ", "") == "" { return false }
    let warnings = strings::warnings();

    let mut args = Vec::<String>::new();
    let mut buffer = Buffer::new(&text);
    
    let mut word = String::new();
    // let mut redirect = String::new();

    while buffer.inBounds() {
        match buffer.getCharacter() {
            ' ' => { replace_push(&mut args, word); word = String::new(); },

            // Quotations
            '"' => { loop { if buffer.advanceNCheck() { match buffer.getCharacter() {
                '\\' => { if !buffer.advanceNCheck() { word.push('\\'); }
                    word += &match buffer.getCharacter() {
                        'n' => '\n',
                        't' => '\t',
                        '\'' => '\'',
                        '\\' => '\\', 
                        a => a,
                    }.to_string()
                },
                '"' => { args.push(word); word = String::new(); buffer.advance(); break; }
                a => word.push(a),
            }} else { quotation_warn(&warnings["doubleQuotePrompt"]); }}},

            '\'' => { loop { if buffer.advanceNCheck() { match buffer.getCharacter() {
                    '\'' => { args.push(word); word = String::new(); buffer.advance(); break; }
                    a => word.push(a),
            }} else { quotation_warn(&warnings["singleQuotePrompt"]); }}},

            /*
            // Redirect output to a file.
            '>' => { if !word.is_empty() { args.push(word.clone()); } if buffer.advanceNCheck() { while buffer.inBounds() {
                redirect.push(buffer.getCharacter()); buffer.advance()
            }} else { println!("{}", errors["missingFileRedirect"]); has_error = true }}
            */

            a => word.push(a),
        }
    buffer.advance(); }
    
    if !word.is_empty() { replace_push(&mut args, word); }
    
    commands::run_command(text, args.iter().map(|s| s.as_str()).collect(), stdout)
}

// yoinked from Duohexyne which was yoinked from Hexagn-rust
pub struct Buffer {
	data: String,
	index: usize
}
#[allow(non_snake_case)]
impl Buffer {
    pub fn new(source: &String) -> Buffer { Buffer { data: source.clone(), index: 0 } }
    pub fn inBounds(&self) -> bool { self.index < self.data.len() }
    pub fn advance(&mut self) { self.index += 1 }
    pub fn advanceNCheck(&mut self) -> bool { self.advance(); self.inBounds() }
    pub fn getCharacter(&self) -> char { let result = match self.data.chars().nth(self.index) { Some(_result) => _result, None => { eprintln!("{}", strings::errors()["bufferGetCharacter"]); exit(-1); } }; result }
}

fn quotation_warn(prompt: &String) -> String {
    print!("{}", prompt); stdout().flush().unwrap();
    "\"".to_string() // later
}

#[allow(deprecated)]
fn replace_push(args: &mut Vec<String>, mut word: String) {
    if word.starts_with("~") { word.replace_range(0..1, &env::home_dir().unwrap().to_string_lossy().to_string()) }
args.push(word) }