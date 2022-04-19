use std::collections::HashSet;

fn is_anagram(mut char_map: Vec<(char, i32)>, anagram: &str) -> bool {
    let char_list = anagram.to_lowercase();

    for character in char_list.to_lowercase().to_string().chars() {
        let index = char_map.iter().position(|tuple| tuple.0 == character);

        match index {
            Some(_) => {
                if char_map.get(index.unwrap()).unwrap().1 > 0 {
                    char_map.get_mut(index.unwrap()).unwrap().1 -= 1;
                } else {
                    return false
                }
            },
            None => return false
        }
    }

    true
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let char_list = word.to_lowercase();
    let mut char_map: Vec<(char, i32)> = Vec::new();
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for character in char_list.chars() {
        let index = char_map.iter().position(|tuple| tuple.0 == character);

        match index {
            Some(_) => char_map.get_mut(index.unwrap()).unwrap().1 += 1,
            None => char_map.push((character, 1))
        }
    };

    for anagram in possible_anagrams {
        if char_list.eq(&(*anagram).to_lowercase()) || word.len() != (*anagram).len() {
            continue;
        }

        if is_anagram(char_map.clone(), anagram) {
            anagrams.insert(anagram);
        }
    }

    anagrams
}
