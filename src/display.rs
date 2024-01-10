use indexmap::IndexMap;
use sdl2::keyboard::Keycode;

const CYAN: &str = "\x1b[1;36m";
const YELLOW: &str = "\x1b[1;33m";
const GREEN: &str = "\x1b[1;32m";
const PURPLE: &str = "\x1b[1;35m";
const END: &str = "\x1b[0m";


pub fn display_keymap(keymap: &IndexMap<Keycode, String>) {
    println!("{}\n------ Key mappings ------\n", PURPLE);
    keymap.iter().for_each(|(key, val)| {
        println!("{}{} {}-> {}{}", CYAN, key, YELLOW, GREEN, val);
    });
    println!("\n{}--------------------------{}\n",PURPLE, END);
}


pub fn display_combo(new_state: &Vec<String>, combo_name: String) {
    new_state.iter().for_each(|action| {
        print!("{}[{}{}{}] {}", YELLOW, CYAN, action, YELLOW, END);
    });
    println!("\n{}{}{}\n", GREEN, combo_name, END);
}