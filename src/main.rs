use indexmap::IndexMap;
use std::{env, fs, process::exit,};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    EventPump,
};
mod parsing;

fn get_file(filename: &String)-> String {
    fs::read_to_string(filename)
        .expect("Error while reading file")
}

fn event_loop(event_pump: &mut sdl2::EventPump) {
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => println!("{:?}", key),
                _ => {}
            }
        }
    }
}

fn init_sdl() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Ft_ality", 800, 600)
        .build()
        .expect("Could not initialize the video subsystem");

    let mut event_pump = sdl_context.event_pump().unwrap();
    event_loop(&mut event_pump)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run [filename]"); exit(1)
    }
    let str_file: String = get_file(&args[1]);
    // dbg!(&str_file); // OK
    parsing::parse_file(&str_file);
    let (keymap, combos) = parsing::parse_file(&str_file);
    dbg!(&keymap); // OK
    dbg!(&combos); // OK
    init_sdl();
}
