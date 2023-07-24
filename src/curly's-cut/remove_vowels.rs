const VOWEL_COUNT: usize = 5;
const VOWELS: [char; VOWEL_COUNT] = ['a', 'e', 'i', 'o', 'u'];

pub fn remove_vowels(s: String) -> String {
    s.chars().filter(|ch| !VOWELS.contains(ch.to_lowercase())).collect()
}


use std::collections::HashSet;
use once_cell::sync::Lazy;

static VOWELS: Lazy<HashSet<char>> = Lazy::new(|| "aeiou".chars().collect());

pub fn remove_vowels(s: String) -> String {
    s.chars().filter(|ch| !VOWELS.contains(ch.to_lowercase())).collect()
}use std::collections::HashSet;
use once_cell::sync::Lazy;

static VOWELS: Lazy<HashSet<char>> = Lazy::new(|| "aeiou".chars().collect());

pub fn remove_vowels(s: String) -> String {
    s.chars().filter(|ch| !VOWELS.contains(ch.to_lowercase())).collect()
}