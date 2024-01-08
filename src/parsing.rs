use std::process::exit;
use indexmap::IndexMap;


fn count_occurrences(src: &str, target: &str) -> usize {
    src.matches(target).count()
}

fn parse_keymap_line(line: &str) -> Option<(String, String)> {
    let split_line: Vec<&str> = line.split(';').collect();
    if count_occurrences(line, ";") == 1 {
        Some((split_line[0].to_string(), split_line[1].to_string()))
    } else {
        println!("Parsing error: line '{}' in 'keymap' should contain one ';'", line);
        exit(1)
    }
}

fn get_keymap(file_keymap: &str) -> IndexMap<String, String> {
    file_keymap.lines().filter_map(parse_keymap_line).collect()
}


// fn get_combos() -> IndexMap<Vec<String>, String> {

// }

fn check_separator(parts: &Vec<&str>) {
    if parts.len() != 2 {
        println!("Parsing error: file should contain one '\\n\\n'");
        exit(1);
    }
}

pub fn parse_file(file: &String) {
    let parts: Vec<&str> = file.split("\n\n").collect();
    check_separator(&parts);
    // dbg!(&parts); // OK
    let keymap = get_keymap(&parts[0]);
    dbg!(&keymap); // OK
}
