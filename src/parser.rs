use termion::raw::RawTerminal;
use std::{io::Stdout, process::exit};
use okeanos::commands;
use okeanos::strings;

pub fn parse(text: String, sign: String, stdout: &mut RawTerminal<Stdout>) -> bool {
    // Return here is for "should_i_exit"
    if text == "" { return true }
    // let warnings = strings::warnings();

    let mut args = Vec::<String>::new();
    let mut buffer = Buffer::new(&text);
    let mut word = String::new();
    while buffer.inBounds() {
        match buffer.getCharacter() {
            ' ' => { args.push(word); word = String::new(); },

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
            }} else { println!("not implemented yet."); return true; }}},

            '\'' => { loop { if buffer.advanceNCheck() { match buffer.getCharacter() {
                    '\'' => { args.push(word); word = String::new(); buffer.advance(); break; }
                    a => word.push(a),
            }} else { println!("not implemented yet."); return true; }}},

            a => word.push(a),
        }
    buffer.advance(); } if !word.is_empty() { args.push(word); }
    commands::run_command(text, args.iter().map(|s| s.as_str()).collect(), sign, stdout)
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