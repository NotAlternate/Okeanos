use std::{io::Stdout, process::exit, env};
use crate::{commands, strings};
use termion::raw::RawTerminal;

pub fn parse(text: String, stdout: &mut RawTerminal<Stdout>) -> bool {
    let errors = strings::errors();

    // Return here is for "should_i_exit"
    if text.replace(" ", "") == "" { return false }

    let mut args = Vec::<String>::new();
    let mut buffer = Buffer::new(&text);
    
    let mut word = String::new();
    let mut redirect = String::new();
    let mut has_error = false;

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
                '"' => { if buffer.advanceNCheck() { match buffer.getCharacter() { ' ' => (), _ => {
                    while buffer.inBounds() { match buffer.getCharacter() { ' ' => break, abc => word.push(abc) } buffer.advance(); }
                }}} args.push(word); word = String::new(); break; }
                
                a => word.push(a),
            }} else { println!("Not implemented"); return false }}},

            '\'' => { loop { if buffer.advanceNCheck() { match buffer.getCharacter() {
                    '\'' => { args.push(word); word = String::new(); break; }
                    a => word.push(a),
            }} else { println!("Not implemented."); return false }}},

            // Redirect output to a file.
            '>' => { if !word.is_empty() { args.push(word.clone()); } if buffer.advanceNCheck() { let mut first = true; while buffer.inBounds() { match buffer.getCharacter() {
                ' ' => { if first { while buffer.inBounds() { match buffer.getCharacter() { ' ' => (), ch => { redirect.push(ch); break; } } buffer.advance() } first = false; } else { redirect.push(' '); } }
                character => { redirect.push(character); }
            } buffer.advance() }} else { println!("{}", errors["missingFileRedirect"]); has_error = true }}

            a => word.push(a),
        }
    buffer.advance(); }
    
    if has_error { return false }
    if !word.is_empty() { replace_push(&mut args, word); }
    
commands::run_command(text, args.iter().map(|s| s.as_str()).collect(), stdout, redirect) }

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

#[allow(deprecated)]
fn replace_push(args: &mut Vec<String>, mut word: String) {
    if word.starts_with("~") { word.replace_range(0..1, &env::home_dir().unwrap().to_string_lossy().to_string()) }
args.push(word) }