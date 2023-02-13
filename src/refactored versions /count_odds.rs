pub fn count_odds(low: i32, high: i32) -> i32 {
    (low..=high).filter(|x| x % 2 == 1).count() as i32
}