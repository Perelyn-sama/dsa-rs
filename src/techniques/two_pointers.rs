// Two pointers
// implemented for palindrome problem
fn two_pointers(s: &str) -> bool {
    let mut left = 0;
    let mut right = s.len() - 1;

    while left <= right {
        if s.chars().nth(left).unwrap() != s.chars().nth(right).unwrap() {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}