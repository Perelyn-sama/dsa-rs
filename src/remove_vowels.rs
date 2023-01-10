// rank - easy
// source - leetcode
pub fn remove_vowels(mut s: String) -> String {
    s.retain(|c| c != 'a' && c != 'e' && c != 'i' && c != 'o'&& c != 'u' );
    s
}