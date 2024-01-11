use indexmap::IndexMap;
use crate::display;

fn find_combo(state: &Vec<String>, combos: &IndexMap<Vec<String>, String>) -> Option<String> {
    combos.iter()
        .find_map(|(keys, value)| {
            if state == (keys) { Some(value.clone()) }
            else { None }
        })
}

fn is_longer_combo(state: &Vec<String>, combos: &IndexMap<Vec<String>, String>) -> bool {
    combos.keys()
          .any(|keys| keys.len() > state.len() && keys.starts_with(state))
}

fn is_in_combo(state: &Vec<String>, combos: &IndexMap<Vec<String>, String>) -> bool {
    combos.keys()
          .any(|keys| keys.starts_with(state))
}

pub fn check_if_combo(state: Vec<String>, action: String, combos: &IndexMap<Vec<String>, String>) -> Vec<String> {
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
