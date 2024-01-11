use indexmap::IndexMap;
use sdl2::{
    event::Event,
    keyboard::Keycode,
};
use crate::combos;

fn handle_key(key: Keycode, state: Vec<String>, keymap: &IndexMap<Keycode, String>, combos: &IndexMap<Vec<String>, String>) -> Vec<String> {
    if let Some(action) = keymap.get(&key) {
        combos::check_if_combo(state, action.clone(), combos)
    } else {
        state
    }
}

fn event_loop(event_pump: &mut sdl2::EventPump, keymap: &IndexMap<Keycode, String>, 
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


pub fn init_sdl(keymap: IndexMap<Keycode, String>, combos: IndexMap<Vec<String>, String>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _window = video_subsystem
        .window("Ft_ality", 800, 600)
        .build()
        .expect("Could not initialize the video subsystem");

    let mut event_pump = sdl_context.event_pump().unwrap();
    event_loop(&mut event_pump, &keymap, &combos, vec![])
}