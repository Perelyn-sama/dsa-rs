// rank - easy
// source - leetcode
pub fn length_of_last_word(s: String) -> i32 {
    let vec: Vec<_> = s.trim().split(" ").collect();
    vec[vec.len() - 1].len() as i32
}