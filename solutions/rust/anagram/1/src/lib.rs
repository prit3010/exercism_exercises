use std::collections::{HashMap, HashSet};

fn char_map(s: &str) -> HashMap<char, i32> {
    let mut m = HashMap::new();
    for c in s.chars() {
        *m.entry(c).or_insert(0) += 1;
    }
    m
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let main_map = char_map(&word_lower);

    let mut set: HashSet<&'a str> = HashSet::new();

    for &candidate in possible_anagrams {
        let cand_lower = candidate.to_lowercase();
        if cand_lower == word_lower {
            continue;
        }
        if char_map(&cand_lower) == main_map {
            set.insert(candidate);
        }
    }

    set
}
