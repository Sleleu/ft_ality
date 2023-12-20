use std::env;
use std::fs;
mod parsing;

fn get_file(filename: &String)-> String {
    fs::read_to_string(filename)
        .expect("Error while reading file")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run [filename]");
    }
    let str_file: String = get_file(&args[1]);
    let (keymap, combos) = parsing::parse_file(&str_file);
}
