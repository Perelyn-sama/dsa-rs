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

#[test]
fn test_pad() {
    let input_1 = "10011".to_string();
    let input_2 = 8;
    let expected = "00010011".to_string();
    assert_eq!(pad(input_1, input_2), expected);

}

#[test]
fn test_chunkify() {
    let input_1 = "this is a test";
    let input_2 = 2;
    let expected = vec!["th", "is", " i", "s ", "a ", "te", "st"];
    assert_eq!(chunkify(input_1, input_2), expected);
}

#[test]
fn test_rotateRight() {
    let input_1 = "1011";
    let input_2 = 3;
    let expected = "0111".to_string();
    assert_eq!(rotateRight(input_1, input_2), expected);
}

#[test]
fn test_preProcess() {
    let input_1 = "Perelyn";
    let expected = "01010000011001010111001001100101011011000111100101101110100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111000".to_string();
    assert_eq!(preProcess(input_1), expected);
}