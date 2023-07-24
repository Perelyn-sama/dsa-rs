pub fn merge_alternately(word1: String, word2: String) -> String {
    
    let mut res = String::new();
    let (mut word1_iter, mut word2_iter) = (word1.chars(), word2.chars());
    loop {
        match (word1_iter.next(), word2_iter.next()) {
            (Some(a), Some(b)) => {
                res.push(a);
                res.push(b);
            }
            (Some(a), None) => res.push(a),
            (None, Some(b)) => res.push(b),
            (None, None) => break res,
        }
    }
}

#[test]
fn test_merge_alternately() {
    // Input: word1 = "abc", word2 = "pqr"
    let result = merge_alternately(String::from("abc"), String::from("pqr"));
    let expected_result = String::from("apbqcr");
    assert_eq!(result, expected_result);
}
