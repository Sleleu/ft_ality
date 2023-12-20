use std::env;
use std::fs;

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
    let keys_values: Vec<&str> = str_file.split('\n').collect();
    println!("File content:\n{:?}", keys_values);
}
