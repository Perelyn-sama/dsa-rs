// Rank - Easy
// Source - Leetcode

pub fn fizz_buzz(n: i32) -> Vec<String> {
    let nums: Vec<i32> = (1..=n).collect();
    let mut num_str: Vec<String> = Vec::new();
    for i in nums {
        match i {
            i if i % 3 == 0 &&  i % 5 == 0 => num_str.push("FizzBuzz".to_string()),
            i if i % 3 == 0 => num_str.push("Fizz".to_string()),
            i if i % 5 == 0  => num_str.push("Buzz".to_string()),
            _ => num_str.push(i.to_string())
        }
    }
    num_str
}