// rank - easy
// source - leetcode - https://leetcode.com/problems/remove-element/description/

// with filter
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.iter().filter(|x| **x != val).count() as i32
}

// with retain
pub fn remove_element_with_retain(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| *x != val);
    nums.len() as i32
}

#[test]
fn test_remove_element() {
    let mut test_vals : Vec<i32> = vec![3,2,2,3];
    let result = remove_element(&mut test_vals, 3);
    let expected_result = 2;

    assert_eq!(result, expected_result);
}

#[test]
fn test_remove_element_with_retain() {
    let mut test_vals : Vec<i32> = vec![3,2,2,3];
    let result = remove_element_with_retain(&mut test_vals, 3);
    let expected_result = 2;

    assert_eq!(result, expected_result);
}

// Input: nums = [3,2,2,3], val = 3
// Output: 2, nums = [2,2,_,_]
// Explanation: Your function should return k = 2, with the first two elements of nums being 2.
// It does not matter what you leave beyond the returned k (hence they are underscores).