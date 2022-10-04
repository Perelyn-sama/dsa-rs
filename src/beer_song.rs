// rank - easy
// source - exercism
pub fn verse(n: u32) -> String {
    match n {
        3.. => format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",
            n - 1
        ),
        2 =>  format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n - 1
        ),
        1 =>  format!(
            "{n} bottle of beer on the wall, {n} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"
        ),
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"
        )
    }
}


pub fn sing(start: u32, end: u32) -> String {
    let mut i = start; // 3
    let mut res = String::from("");

    if end == 0 {
        while i == 0 || i > end {
            if i == 0 {
                res = res + &verse(i);
                break;
            } else {
                res = res + &verse(i) + "\n";
                i -= 1;
            }
        }
    } else {
        while i > end - 1 {
            if i == end {
                res = res + &verse(i);
                break;
            } else {
                res = res + &verse(i) + "\n";
                i -= 1;
            }
        }
    }

    res
}