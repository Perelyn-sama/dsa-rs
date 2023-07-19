// rank - easy
// source - leetcode
pub fn merge_alternately(word1: String, word2: String) -> String {
    let total_length = word1.len() + word2.len();
    let mut word1_rev = word1.chars().rev().collect::<String>();
    let mut word2_rev = word2.chars().rev().collect::<String>();
    let mut res = String::new();

    for i in 0..total_length as usize {
        if word1_rev.len() > 0 {
            res.push(word1_rev.pop().unwrap());
        }
        if word2_rev.len() > 0 {
            res.push(word2_rev.pop().unwrap());
        }
    }
    res
}

#[test]
fn test_merge_alternately() {
    // Input: word1 = "abc", word2 = "pqr"
    let result = merge_alternately(String::from("abc"), String::from("pqr"));
    let expected_result = String::from("apbqcr");
    assert_eq!(result, expected_result);
}
