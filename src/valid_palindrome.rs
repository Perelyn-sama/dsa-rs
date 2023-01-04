pub fn is_palindrome(s: String) -> bool {
    let mut clean_string: String = String::new();
    for i in s.chars() {
        match i {
            i if i.is_alphanumeric() => clean_string.push(i),
            _ => ()
        }
    }
    let lowercase_string = clean_string.to_lowercase();
    let mut to_vec = lowercase_string.chars().collect::<Vec<_>>();
    to_vec.reverse();
    let reversed_string = to_vec.iter().collect::<String>();

    lowercase_string == reversed_string
}