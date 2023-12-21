use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process::exit;
use std::env;
use std::fs;
mod parsing;

fn get_file(filename: &String)-> String {
    fs::read_to_string(filename)
        .expect("Error while reading file")
}

fn print_keys(c: Key) -> bool {
    match c {
        Key::Char(c) => println!("{:?}\r", Key::Char(c)),
        Key::Ctrl(c) => println!("{:?}\r", Key::Ctrl(c)),
        Key::Esc => return true,
        _ => (),
    }
    false
}

fn main_loop() {
    let stdout = stdout().into_raw_mode().unwrap();
    println!("Welcome !\n\r");

    for c in stdin().keys() {
        match c {
            Ok(c) => if print_keys(c) {break},
            Err(c) => (),
        }
    }
    println!("End !\r");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run [filename]");
    }
    let str_file: String = get_file(&args[1]);
    let (keymap, combos) = parsing::parse_file(&str_file);
    main_loop();
    }