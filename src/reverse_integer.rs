// rank - medium
// source - leetcode
pub fn reverse(mut x: i32) -> i32 {
    let mut vec = Vec::new();
    let len = if x > 0 {
        x.to_string().len()
    } else {
        x.to_string().len() - 1
    };
    for _i in 0..len {
        vec.push(&x % 10);
        x = &x / 10;
    }
    vec.iter().fold(0, |acc, &x| match acc.checked_mul(10) {
        Some(integer) => integer + x,
        None => acc.checked_mul(10).unwrap_or(0),
    })
}