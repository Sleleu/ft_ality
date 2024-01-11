use std::{env, fs, process::exit};

pub fn get_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(file_str) => { file_str }
        Err(e) => {
            println!("Filename error: {}", e);
            exit(1);
        }
    }
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
