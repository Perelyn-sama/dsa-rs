fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .fold(0, |sum, stone| sum + jewels.contains(stone) as i32)
}
