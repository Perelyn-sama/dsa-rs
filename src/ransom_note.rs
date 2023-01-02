// rank - easy
// source - leetcode
fn get_frequency(string: String) -> HashMap<char, i32> {
    let letters: Vec<char> = string.chars().collect();
    let mut count = HashMap::new();
    for letter in letters {
        *count.entry(letter).or_insert(0) += 1;
    }
    count
}

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut ransom_freq: HashMap<char, i32> = get_frequency(ransom_note);
    // Try to decrement the value of a key, when we spot the letter in `magazine`
    for i in magazine.chars() {
        match ransom_freq.get(&i) {
            Some(_char) => {
                let count = ransom_freq.entry(i).or_insert(0);
                *count -= 1;
            }
            None => (),
        }
    }
    dbg!(&ransom_freq);
    // Determine if the sum of all the values of the hashmap are equal to zero
    let mut sum: i32 = 0;
    for mut val in ransom_freq.values() {
        if val < &0 {
            val = &0;
            sum += val;
        }
        sum += val;
    }
    dbg!(sum);
    sum == 0
}
