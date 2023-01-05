// rank - easy
// source - leetcode
pub fn sort_string(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}

pub fn is_anagram(s: String, t: String) -> bool {
    sort_string(s) == sort_string(t)
}