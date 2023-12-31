use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_split: Vec<char> = word_lower.chars().collect();
    word_split.sort_unstable();
    return possible_anagrams
        .iter()
        .fold(HashSet::new(), |mut acc: HashSet<&'a str>, e: &&str| {
            let e_lower = e.to_lowercase();
            let mut element_split: Vec<char> = e_lower.chars().collect();
            element_split.sort_unstable();

            if word_split == element_split && word_lower != *e_lower {
                acc.insert(e);
            };
            return acc;
        });
}
