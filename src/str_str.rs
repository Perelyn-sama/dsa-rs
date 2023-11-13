// rank - easy
// source - leetcode - https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
pub fn str_str(haystack: String, needle: String) -> i32 {
    return match haystack.as_str().find(needle.as_str()) {
        Some(index) =>  index as i32,
        None => -1
    };
}

#[test]
pub fn test_one_str_str() {
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();

    let result = str_str(haystack, needle);
    let expected_result = 0;

    assert_eq!(result, expected_result);
}

#[test]
pub fn test_two_str_str() {
    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();

    let result = str_str(haystack, needle);
    let expected_result = -1;

    assert_eq!(result, expected_result);
}