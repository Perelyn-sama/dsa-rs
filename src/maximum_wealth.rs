// rank - easy
// source - exercism
pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut vec_of_sums = Vec::new();
    for i in accounts {
        vec_of_sums.push(i.iter().sum());
    }
    *vec_of_sums.iter().max().unwrap()
}