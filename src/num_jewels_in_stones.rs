// rank - easy
// source - exercism
pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut sum = 0;
    for i in stones.chars() {
        if jewels.contains(i) {
            sum += 1;
        }
    }
    sum
}
