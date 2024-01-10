use std::process::exit;
use indexmap::IndexMap;

use sdl2::keyboard::Keycode;

fn count_occurrences(src: &str, target: &str) -> usize {
    src.matches(target).count()
}

fn parse_keymap_line(line: &str) -> Option<(Keycode, String)> {
    let split_line: Vec<&str> = line.split(';').collect();
    if count_occurrences(line, ";") == 1 {
        if let Some(keycode) = Keycode::from_name(split_line[0]) {
            return Some((keycode, split_line[1].to_string()));
        } else {
            println!("Parsing error: Invalid keycode '{}'", split_line[0]);
            exit(1)
        }
    } else {
        println!("Parsing error: line '{}' in 'keymap' should contain one ';'", line);
        exit(1)
    }
}

// fn parse_keymap_line(line: &str) -> Option<(String, String)> {
//     let split_line: Vec<&str> = line.split(';').collect();
//     if count_occurrences(line, ";") == 1 {
//         Some((split_line[0].to_string(), split_line[1].to_string()))
//     } else {
//         println!("Parsing error: line '{}' in 'keymap' should contain one ';'", line);
//         exit(1)
//     }
// }

fn parse_combo_line(line: &str) -> Option<(Vec<String>, String)> {
    let split_line: Vec<&str> = line.split(';').collect();
    if count_occurrences(line, ";") == 1 {
        let moves: Vec<String> = split_line[0].split('+').map(String::from).collect();
        let combo = split_line[1].to_string();
        Some((moves, combo))
    } else {
        println!("Parsing error: line '{}' in 'combos' should contain one ';'", line);
        exit(1)
    }
}

fn get_keymap(file_keymap: &str) -> IndexMap<Keycode, String> {
    file_keymap.lines().filter_map(parse_keymap_line).collect()
}

fn get_combos(file_combos: &str) -> IndexMap<Vec<String>, String> {
    file_combos.lines().filter_map(parse_combo_line).collect()
}

fn check_separator(parts: &Vec<&str>) {
    if parts.len() != 2 {
        println!("Parsing error: file should contain one '\\n\\n'");
        exit(1);
    }
}

pub fn parse_file(file: &String)-> (IndexMap<Keycode,String>, IndexMap<Vec<String>,String>) {
    let parts: Vec<&str> = file.split("\n\n").collect();
    check_separator(&parts);
    // dbg!(&parts); // OK
    let keymap = get_keymap(&parts[0]);
    // dbg!(&keymap); // OK
    let combos = get_combos(&parts[1]);
    // dbg!(&combos); // OK
    (keymap, combos)
}
