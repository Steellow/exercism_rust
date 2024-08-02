use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&'a str> = HashSet::new();

    let mut sorted_word: Vec<char> = word.to_lowercase().chars().collect();
    sorted_word.sort_unstable();
 
    for &x in possible_anagrams {
        if x.len() != word.len() || x.to_lowercase() == word.to_lowercase() {
           continue; 
        }

        let mut sorted_x: Vec<char> = x.to_lowercase().chars().collect();
        sorted_x.sort_unstable();

        if sorted_word == sorted_x {
            set.insert(x);
        }
    }

    set
}
