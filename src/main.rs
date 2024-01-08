use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process::exit;
use std::env;
use std::fs;
use indexmap::IndexMap;
mod parsing;

fn get_file(filename: &String)-> String {
    fs::read_to_string(filename)
        .expect("Error while reading file")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run [filename]"); exit(1)
    }
    let str_file: String = get_file(&args[1]);
    // dbg!(&str_file); // OK
    parsing::parse_file(&str_file);
    // let Ok((keymap, combos)) = parsing::parse_file(&str_file)
    //     else {
    //         println!("Parsing Error"); exit(1) 
    //     };
    // listen_moves(keymap, combos);
}
