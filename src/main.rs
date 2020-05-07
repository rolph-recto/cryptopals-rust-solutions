mod encoding;

use encoding::{Hex, Base64};

// set 1 challenge 1: convert hex to base64
fn set1_challenge1() {
    let s: Hex = Hex::new("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let result: Base64 = s.to_base64();

    println!("hex: {0}\nexpected base64: {1}\nreceived base64: {2}", s, expected, result);
    if result.to_string() == expected {
        println!("set 1 challenge 1 passed");

    } else {
        println!("set 1 challenge 1 failed");
    }
}

// set 1 challenge 2: XOR two hex buffers
fn set1_challenge2() {
    let buf1: Hex = Hex::new("1c0111001f010100061a024b53535009181c");
    let buf2: Hex = Hex::new("686974207468652062756c6c277320657965");
    let expected: &str = "746865206b696420646f6e277420706c6179";
    let result: Hex = buf1.xor(&buf2);

    println!("buffer 1: {0}\nbuffer 2: {1}\nexpected: {2}\nresulted: {3}",
        buf1, buf2, expected, result);

    if expected == result.to_string() {
        println!("set 1 challenge 2 passed");

    } else {
        println!("set 1 challenge 2 failed");
    }
}


// encrypt ascii text with a 1-byte key
fn xor_byte_cipher(s: &str, k: char) -> String {
    let mut ciphertext: String = String::new();

    for c in s.chars() {
        ciphertext.push(((c as u8) ^ (k as u8)) as char);
    }

    return ciphertext;
}

fn set1_challenge3() {
    let s: Hex = Hex::new("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let s_ascii: String = s.to_ascii();

    for k in 0u8..255 {
        let ciphertext: String = xor_byte_cipher(&s_ascii, k as char);
        println!("s: {0}\nk: {1}\nc: {2}", s_ascii, k, ciphertext);
    }
}

fn main() {
    set1_challenge1();
    set1_challenge2();
    set1_challenge3();
}
