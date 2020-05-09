mod encoding;

use encoding::{Hex, Base64};
use std::collections::HashMap;

// set 1 challenge 1: convert hex to base64
fn set1_challenge1() {
    let s: Hex = Hex::new("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let result: Base64 = s.to_base64();

    println!("hex: {0}\nexpected base64: {1}\nreceived base64: {2}", s, expected, result);
    if *result == expected {
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

    if expected == *result {
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

fn make_char_count_map(s: &str) -> HashMap<char, u32> {
    let mut char_freq_map: HashMap<char, u32> = HashMap::new();
    for c in s.chars().filter(|c: &char| c.is_ascii_alphabetic()) {
        if !char_freq_map.contains_key(&c) {
            char_freq_map.insert(c, 1);

        } else {
            char_freq_map.insert(c, char_freq_map.get(&c).unwrap() + 1);
        }
    }

    return char_freq_map;
}

// set 1 challenge 2: determine 1-byte encryption key
fn set1_challenge3() {
    let english_letter_freq: HashMap<char, f64> =
        [
            ('e', 0.1202),
            ('t', 0.0910),
            ('a', 0.0812),
            ('o', 0.0768),
            ('i', 0.0731),
            ('n', 0.0695),
            ('s', 0.0628),
            ('r', 0.0602),
            ('h', 0.0592),
            ('d', 0.0432),
            ('l', 0.0398),
            ('u', 0.0288),
            ('c', 0.0271),
            ('m', 0.0261),
            ('f', 0.0230),
            ('y', 0.0211),
            ('w', 0.0209),
            ('g', 0.0203),
            ('p', 0.0182),
            ('b', 0.0149),
            ('v', 0.0111),
            ('k', 0.0069),
            ('x', 0.0017),
            ('q', 0.0011),
            ('j', 0.0010),
            ('z', 0.0007),
        ].iter().cloned().collect();

    let s: Hex = Hex::new("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let s_ascii: String = s.to_ascii();

    let mut best_freq_diff: f64 = -1.0;
    let mut best_char: char = '0';
    let mut best_ciphertext: String = "".to_string();

    for k in 0u8..255 {
        let ciphertext: String = xor_byte_cipher(&s_ascii, k as char);
        let ciphertext_lower: String = ciphertext.to_ascii_lowercase();
        let char_count_map: HashMap<char, u32> = make_char_count_map(&ciphertext_lower);
        let char_freq_map: HashMap<char, f64> =
            char_count_map
            .iter()
            .map(|(c, n)| (*c, (*n as f64) / (ciphertext.len() as f64)))
            .collect();

        let mut freq_diff: f64 = 0.0;
        for (&c, &freq) in english_letter_freq.iter() {
            // if no character occurred in the ciphertext, set frequency difference to 1.0
            // this creates a penalty against ciphertexts that don't have letters
            freq_diff += f64::abs(freq - char_freq_map.get(&c).unwrap_or(&1.0));
        }

        if freq_diff < best_freq_diff || best_freq_diff < 0.0 {
            best_freq_diff = freq_diff;
            best_char = k as char;
            best_ciphertext = ciphertext;
        }
    }

    let expected_key: &str = "X";
    let expected_ciphertext: &str = "Cooking MC's like a pound of bacon";
    println!("expected key: {0}; actual ciphertext: {1}", expected_key, expected_ciphertext);
    println!("actual key: {0}; actual ciphertext: {1}", best_char, best_ciphertext);

    if expected_key == best_char.to_string() && expected_ciphertext == best_ciphertext {
        println!("set 1 challenge 3 passed");

    } else {
        println!("set 1 challenge 3 failed");
    }
}

fn main() {
    set1_challenge1();
    set1_challenge2();
    set1_challenge3();
}
