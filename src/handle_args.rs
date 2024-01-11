use std::{env, fs, process::exit};

pub fn get_file(filename: &String)-> String {
    fs::read_to_string(filename)
        .expect("Error while reading file")
}

pub fn check_arg_len(len: usize) {
    match len {
        2 => {},
        _ => {
            println!("Usage: cargo run [filename]");
            exit(1)
        }
    }
}

pub fn get_args() -> Vec<String> {
    env::args().collect()
}
