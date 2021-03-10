use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let lower_case_word = word.to_lowercase();
    let mut word_chars: Vec<char> = lower_case_word.chars().collect::<Vec<char>>();
    word_chars.sort();

    for anagram in possible_anagrams {
        let lower_case_anagram = anagram.to_lowercase();
        if lower_case_word != lower_case_anagram {
            let mut anagram_chars: Vec<char> = lower_case_anagram.chars().collect::<Vec<char>>();
            anagram_chars.sort();

            if anagram_chars.len() == word_chars.len() && word_chars == anagram_chars {
                result.insert(*anagram);
            }
        }
    }

    result
}
