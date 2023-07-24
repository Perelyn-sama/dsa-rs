use std::collections::VecDeque;

pub fn reverse(mut x: i32) -> Option<i32> {
    let mut reversed = VecDeque::new();
    loop {
        reversed.push_back(x % 10);
        x /= 10;
        if x == 0 {
            break;
        }
    }

    reversed.into_iter().try_fold(0_i32, |acc, el| {
        acc.checked_mul(10).and_then(|n| n.checked_add(el))
    })
}