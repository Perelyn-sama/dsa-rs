use std::u128;

pub fn pad(str: String, bits: u32) -> String {
    let mut res = str.chars().rev().collect::<String>();

    while res.len() as u32 % bits != 0 {
        res.push('0');
    }
    res.chars().rev().collect::<String>()
}

pub fn chunkify(str: String, size: usize) -> Vec<String> {
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

pub fn sha256(message: &str ) ->  String {

    let H0 = "0x6a09e667";
    let H1 = "0xbb67ae85";
    let H2 = "0x3c6ef372";
    let H3 = "0xa54ff53a";
    let H4 = "0x510e527f";
    let H5 = "0x9b05688c";
    let H6 = "0x1f83d9ab";
    let H7 = "0x5be0cd19";

    let bits = preProcess(message);
    // println!("{}", bits);
    let chunks = chunkify(bits, 512);
    // println!("{:?}", chunks);

    chunks.iter().enumerate().for_each(|(index, val)| {
        let mut words = chunkify(val.to_string(), 32);

        for i in 16..64 {
            println!("{}", i);
            let W1 = &words[i - 15];
            let W2 = &words[i - 2];
            let R1 = rotateRight(&W1, 7);
            let R2 = rotateRight(&W1, 18);
            let R3 = rotateRight(&W2, 17);
            let R4 = rotateRight(&W2, 19);
            // println!("{}", R1);
            let S0 = u32::from_str_radix(&R1, 2).unwrap() ^ u32::from_str_radix(&R2, 2).unwrap() ^ (u32::from_str_radix(&W1, 2).unwrap() >> 3);

            let S1 = u32::from_str_radix(&R3, 2).unwrap() ^ u32::from_str_radix(&R4, 2).unwrap() ^ (u32::from_str_radix(&W2, 2).unwrap() >> 10);

            let val : u128 = u128::from_str_radix(&words[i - 16], 2).unwrap() + S0 as u128 + u128::from_str_radix(&words[i - 7], 2).unwrap() + S1 as u128;

            // words[i] = pad(format!("{:b}", val >> 0), 32);

            words.push(pad(format!("{:b}", val >> 0), 32));
        }
    });

    "I'm sorry".to_string()
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
    let input_1 = "this is a test".to_string();
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