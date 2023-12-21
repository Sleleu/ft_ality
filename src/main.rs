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

fn print_keys(c: Key) -> Result<(), ()> {
    match c {
        Key::Char(c) => println!("{:?}\r", c),
        Key::Esc => return Err(()),
        _ => (),
    }
    Ok(())
}

fn listen_moves() {
    let stdout = stdout().into_raw_mode();
    let result = stdin().keys().try_for_each(|c| { c.map_err(|_| ()).and_then(print_keys) });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { panic!("Usage: cargo run [filename]"); }
    let str_file: String = get_file(&args[1]);
    let (keymap, combos) = parsing::parse_file(&str_file);
    listen_moves();
}


/* SLD2 TEST */
// extern crate sdl2; 

// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::EventPump;
// use std::env;
// use std::fs;

// mod parsing;

// fn get_file(filename: &String) -> String {
//     fs::read_to_string(filename)
//         .expect("Error while reading file")
// }

// fn event_loop(event_pump: &mut sdl2::EventPump) {
//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit {..} |
//                 Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
//                 Event::KeyDown { keycode: Some(key), .. } => println!("{:?}", key),
//                 _ => {}
//             }
//         }
//     }
// }

// fn init_sdl() {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();
//     let window = video_subsystem
//         .window("Ft_ality", 100, 100)
//         .build()
//         .expect("Could not initialize the video subsystem");

//     let mut event_pump = sdl_context.event_pump().unwrap();
//     event_loop(&mut event_pump)
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 2 {
//         panic!("Usage: cargo run [filename]");
//     }
//     let str_file: String = get_file(&args[1]);
//     let (keymap, combos) = parsing::parse_file(&str_file);
//     init_sdl();
// }
