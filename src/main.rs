use std::env;
use std::fs;

fn get_file(filename: &String)-> String {
    fs::read_to_string(filename)
        .expect("Error while reading file")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("yoyoyo");
    }
    println!("{:?} {:?}", args, args.len());
    let file = get_file(&args[1]);
    println!("File content:\n{}", file)
}
