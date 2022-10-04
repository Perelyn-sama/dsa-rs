// rank - easy
// source - exercism
pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    let mut input = String::from(input);
    let mut o = String::from("");

    loop {
        match input.pop() {
            Some(char) => o.push(char),
            None => break
        }
    }

    o
}