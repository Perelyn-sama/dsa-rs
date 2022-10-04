// rank - easy
// link - https://leetcode.com/explore/interview/card/google/67/sql-2/3044/
// source - leetcode
use std::collections::HashSet;

pub fn run(mails: Vec<&str>) -> i32 {
    let mut results: Vec<String> = Vec::new();
    let mut results_set = HashSet::new();

    for mail in mails {
        let v1: Vec<&str> = mail.split('@').collect();

        let mut after_at_vec : Vec<&str> = vec!["@", v1[1]];
        let mut after_at_str = after_at_vec.concat();

        let v2: Vec<&str> = v1[0].split('.').collect();

        let str_without_dot = v2.concat();
        let mut v3: Vec<&str> = str_without_dot.split('+').collect();

        let mut before_at_str_final = v3[0].to_string();
        before_at_str_final.push_str(&after_at_str[..]);

        results_set.insert(before_at_str_final);
    }

    results_set.len() as i32
}

