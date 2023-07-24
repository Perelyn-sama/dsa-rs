pub fn is_palindrome(s: String) -> bool {
    // step 1
    let clean_string = s
        // iterate over each character in `s`
        .chars()
        // remove non-alphanumerics
        .filter(|ch| ch.is_alphanumeric())
        // convert char to lower case char(s)
        .flat_map(|ch| ch.to_lowercase())
        // allocate the processed chars
        .collect::<Vec<_>>();

    // step 2
    clean_string == clean_string.clone().reverse()
}