use indexmap::IndexMap;
use std::{env, fs, process::exit,};
// use tailcall::tailcall;

extern crate sdl2; 
use sdl2::{
    event::Event,
    keyboard::Keycode,
};
mod parsing;
mod display;

fn get_file(filename: &String)-> String {
    fs::read_to_string(filename)
        .expect("Error while reading file")
}

fn find_combo(state: &Vec<String>, combos: &IndexMap<Vec<String>, String>) -> Option<String> {
    combos.iter()
        .find_map(|(keys, value)| {
            if state == (keys) { Some(value.clone()) }
            else { None }
        })
}

fn is_longer_combo(state: &Vec<String>, combos: &IndexMap<Vec<String>, String>) -> bool {
    combos.keys().any(|keys| keys.len() > state.len() && keys.starts_with(state))
}

fn is_in_combo(state: &Vec<String>, combos: &IndexMap<Vec<String>, String>) -> bool {
    combos.keys().any(|keys| keys.starts_with(state))
}

fn check_if_combo(state: Vec<String>, action: String, combos: &IndexMap<Vec<String>, String>) -> Vec<String> {
    let new_state = vec![state.clone(), vec![action.clone()]].concat();
    match find_combo(&new_state, combos) {
        Some(combo_name) => {
            display::display_combo(&new_state, combo_name);
            if !is_longer_combo(&new_state, combos) { vec![] }
            else { new_state }
        },
        None => {
            if let Some(combo_name) = combos.get(&vec![action.clone()]) { // si spam un combo avec 1 move
                display::display_combo(&vec![action.clone()], combo_name.to_string());
                vec![action] // Retoune l'action
            }
            else if !is_in_combo(&new_state, combos) { vec![] }
            else { new_state }
        }
    }
}

fn handle_key(key: Keycode, state: Vec<String>, keymap: &IndexMap<String, String>, combos: &IndexMap<Vec<String>, String>) -> Vec<String> {
    if let Some(action) = keymap.get(&key.name().to_string()) {
        check_if_combo(state, action.clone(), combos)
    } else {
        state
    }
}

fn event_loop(event_pump: &mut sdl2::EventPump, keymap: &IndexMap<String, String>, 
              combos: &IndexMap<Vec<String>, String>, mut state: Vec<String>) {
    loop {
        if let Some(event) = event_pump.poll_iter().next() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
                Event::KeyDown { keycode: Some(key), .. } => {
                    state = handle_key(key, state, keymap, combos);
                },
                _ => {}
            }}}
}

// version tailcall mais j'arrive pas a supp les warnings :(
// #[tailcall]
// fn event_loop_rec(event_pump: &mut sdl2::EventPump, keymap: &IndexMap<String, String>,
//                         combos: &IndexMap<Vec<String>, String>, state: Vec<String>) {
//     if let Some(event) = event_pump.poll_iter().next() {
//         match event {
//             Event::Quit {..} |
//             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
//             Event::KeyDown { keycode: Some(key), .. } => {
//                 let new_state = handle_key(key, state, keymap, combos);
//                 event_loop_recursive(event_pump, keymap, combos, new_state);
//             },
//             _ => event_loop_recursive(event_pump, keymap, combos, state),
//         }
//     }
//     event_loop_recursive(event_pump, keymap, combos, state);
// }


fn init_sdl(keymap: IndexMap<String, String>, combos: IndexMap<Vec<String>, String>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _window = video_subsystem
        .window("Ft_ality", 800, 600)
        .build()
        .expect("Could not initialize the video subsystem");

    let mut event_pump = sdl_context.event_pump().unwrap();
    event_loop(&mut event_pump, &keymap, &combos, vec![])
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
    // dbg!(&keymap); // OK
    // dbg!(&combos); // OK
    display::display_keymap(&keymap);
    init_sdl(keymap, combos);
}
