// rank - easy
// source - exercism
pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2_u64.pow(&s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    let mut accum = 0;
    for i in 1..=64{
        accum += square(i)
    }
    accum
}

fn process_square_case(input: u32, expected: u64) {
    assert_eq!(square(input), expected);
}
#[test]
/// 1
fn test_1() {
    process_square_case(1, 1);
    // (0, 1)
}
#[test]
//#[ignore]
/// 2
fn test_2() {
    process_square_case(2, 2);
    // (1, 2) = 2
}
#[test]
//#[ignore]
/// 3
fn test_3() {
    process_square_case(3, 4);
    // (2, 4)
}
#[test]
//#[ignore]
/// 4
fn test_4() {
    process_square_case(4, 8);
    // s = 4; s - 1; 2 ^ s == 8
    // (3, 8)
}

//NEW
#[test]
//#[ignore]
/// 16
fn test_16() {
    process_square_case(16, 32_768);
}
#[test]
//#[ignore]
/// 32
fn test_32() {
    process_square_case(32, 2_147_483_648);
}
#[test]
//#[ignore]
/// 64
fn test_64() {
    process_square_case(64, 9_223_372_036_854_775_808);
}
#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn test_square_0_raises_an_exception() {
    square(0);
}
#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn test_square_greater_than_64_raises_an_exception() {
    square(65);
}
#[test]
//#[ignore]
fn test_returns_the_total_number_of_n_the_board() {
    assert_eq!(total(), 18_446_744_073_709_551_615);
}
