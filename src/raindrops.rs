use std::collections::HashSet;
pub fn raindrops(n: u32) -> String {
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0 {
        return n.to_string();
    }
    let mut v = factors(n);

    dbg!(retain_uniq(&mut v));
    dbg!(&v);

    let mut string = String::new();

    for i in v {
        match i {
            3 => string.push_str("Pling"),
            5 => string.push_str("Plang"),
            7 => string.push_str("Plong"),
            _ => (),
        }
    }
    string
}

fn retain_uniq<T: Copy + PartialEq>(nums: &mut Vec<T>) {
    let mut previous = None;
    nums.retain(|&x| Some(x) != std::mem::replace(&mut previous, Some(x)))
}

pub fn factors(mut n: u32) -> Vec<u32> {
    let mut divisor: u32 = 2;
    let mut factors = Vec::new();

    while n != 1 {
        if n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        } else {
            divisor += 1;
        }
    }
    factors
}

#[test]
fn test_1() {
    assert_eq!("1", raindrops(1));
}
#[test]
//#[ignore]
fn test_3() {
    assert_eq!("Pling", raindrops(3));
}
#[test]
//#[ignore]
fn test_5() {
    assert_eq!("Plang", raindrops(5));
}
#[test]
//#[ignore]
fn test_7() {
    assert_eq!("Plong", raindrops(7));
}
#[test]
//#[ignore]
fn test_6() {
    assert_eq!("Pling", raindrops(6));
}
#[test]
//#[ignore]
fn test_8() {
    assert_eq!("8", raindrops(8));
}
#[test]
//#[ignore]
fn test_9() {
    assert_eq!("Pling", raindrops(9));
}
#[test]
//#[ignore]
fn test_10() {
    assert_eq!("Plang", raindrops(10));
}
#[test]
//#[ignore]
fn test_14() {
    assert_eq!("Plong", raindrops(14));
}
#[test]
//#[ignore]
fn test_15() {
    assert_eq!("PlingPlang", raindrops(15));
}
#[test]
//#[ignore]
fn test_21() {
    assert_eq!("PlingPlong", raindrops(21));
}
#[test]
//#[ignore]
fn test_25() {
    assert_eq!("Plang", raindrops(25));
}
#[test]
//#[ignore]
fn test_27() {
    assert_eq!("Pling", raindrops(27));
}
#[test]
//#[ignore]
fn test_35() {
    assert_eq!("PlangPlong", raindrops(35));
}
#[test]
//#[ignore]
fn test_49() {
    assert_eq!("Plong", raindrops(49));
}
#[test]
//#[ignore]
fn test_52() {
    assert_eq!("52", raindrops(52));
}
#[test]
//#[ignore]
fn test_105() {
    assert_eq!("PlingPlangPlong", raindrops(105));
}
#[test]
//#[ignore]
fn test_3125() {
    assert_eq!("Plang", raindrops(3125));
}
#[test]
//#[ignore]
fn test_12121() {
    assert_eq!("12121", raindrops(12_121));
}
