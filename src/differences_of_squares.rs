// rank - easy
// source exercism
pub fn square_of_sum(mut n: u32) -> u32 {
    let mut sum: u32 = 0;
    loop {
        match n {
            0 => {
                break;
            }
            1.. => {
                sum += n;
                n -= 1;
            }
        }
    }
    sum.pow(2)
}

pub fn sum_of_squares(mut n: u32) -> u32 {
    let mut sum: u32 = 0;
    loop {
        match n {
            0 => {
                break;
            }
            1.. => {
                sum += n.pow(2);
                n -= 1;
            }
        }
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}