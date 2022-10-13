// rank - easy 
// source - exercism 
pub fn nth(n: u32) -> u32 {
        let mut count = 0;
    let mut num = 2;

    while count != n as usize {
        num = get_next_prime_number(num as usize);
        count+=1;
    }

    num as u32
}



fn get_next_prime_number(n: usize) -> u32 {
    if n == 0 {
        return 0 as u32;
    }
    let mut v = Vec::new();
    for i in 1 + n..n * n {
        if is_prime(i) {
            v.push(i);
            break;
        }
    }
    v[0] as u32
}

fn is_prime(n: usize) -> bool {
    let mut status = false;
    for i in 2..n {
        if n % i == 0 {
            status = false;
            break;
        } else {
            status = true;
        }
    }
    status
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}
#[test]
// #[ignore]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}
#[test]
// #[ignore]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}
#[test]
// #[ignore]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}
