use indexmap::IndexMap;

// Parsing :
//      - chaque fichier suit : <keymap>""<moves>, tout newline supplémentaire ou autre = erreur
//      - chaque ligne doit contenir un séparateur ';', et contenir du texte avant et après
//      - Tout move est composé des actions du keymapping, et peut contenir un '+' pour définir un ensemble de moves, sinon erreur
//      - Aucun duplicata autorisé sur les K,V de keymap, ou K,V de moves

fn get_keymap() -> IndexMap<String, String> {
    [
        ("d".to_string(), "right".to_string()),
        ("a".to_string(), "left".to_string()),        
        (" ".to_string(), "jump".to_string()),
        ("j".to_string(), "kick".to_string()),
        ("k".to_string(), "punch".to_string()),
    ].iter().cloned().collect()
}

fn get_combos() -> IndexMap<Vec<String>, String> { //hardcode de l'enfer
    [
        (vec!["jump".to_string(), "kick".to_string()], "air kick".to_string()),
        (vec!["jump".to_string(), "punch".to_string()], "air punch".to_string()),
        (vec!["right".to_string(), "kick".to_string()], "big kick".to_string()),
        (vec!["kick".to_string()], "fast kick".to_string()),
    ].iter().cloned().collect()
}

pub fn parse_file(file: &String)-> (IndexMap<String, String>,IndexMap<Vec<String>, String>) {
    let keys_values: Vec<&str> = file.split('\n').collect();
    let keymap = get_keymap();
    dbg!(&keymap);
    let combos = get_combos();
    dbg!(&combos);
    (keymap, combos)
}