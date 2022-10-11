pub fn pad(str: &str, bits: u32) -> String {
    let mut res = str.chars().rev().collect::<String>();

    while res.len() as u32 % bits != 0 {
        res.push('0');
    }
    res.chars().rev().collect::<String>()
}

pub fn chunkify(str: &str, size: usize) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut i = 0;

    while i < str.len() {
        chunks.push((&str[i..i + size]).to_string());
        i += size;
    }
    chunks
}

pub fn rotateRight(bits: &str, turns: usize) -> String {
    bits[bits.len() - turns..].to_owned() + &bits[0..bits.len() - turns]
}

pub fn preProcess(message: &str) -> String {

    let mut v: Vec<String> = Vec::new();

    for char in message.chars() {
        v.push(pad(format!("{:b}", char as u32), 8));
    }

    v.push('1'.to_string());

    let mut m = v.join("");

    while m.len() % 512 != 448 {
        m.push('0');
    }

    let ml = format!("{:b}", message.len() * 8);
    let ml = pad(ml, 8);
    let ml = "0".repeat(64 - ml.len()) + &ml;

    m + &ml
}

