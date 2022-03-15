// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_list: HashMap<&str, i8> = HashMap::new();

    for word in note {
        if !word_list.contains_key(word) {
            word_list.insert(word, 1);
        } else {
            if let Some(value) = word_list.get_mut(word) {
                *value += 1;
            }
        }
    }

    for word in magazine {
        if word_list.contains_key(word) {
            if let Some(value) = word_list.get_mut(word) {
                *value -= 1;
            }
        }
    }

    for value in word_list.values() {
        if *value > 0 {
            return false;
        }
    }

    true
}
