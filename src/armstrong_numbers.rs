// rank - easy
// source - exercism
pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    let mut num_string = String::from(num.to_string());
    let mut sum: u32 = 0;
    let len = num_string.len() as u32;

    loop {
        match num_string.pop() {
            Some(char) => sum = sum + char.to_digit(16).unwrap().pow(len),
            None => break
        }
    }

    num == sum
}