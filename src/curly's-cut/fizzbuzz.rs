pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|num| match num {
            i if i % 3 == 0 && i % 5 == 0 => "FizzBuzz".to_string(),
            i if i % 3 == 0 => "Fizz".to_string(),
            i if i % 5 == 0 => "Buzz".to_string(),
            i => i.to_string(),
        })
        .collect()
}