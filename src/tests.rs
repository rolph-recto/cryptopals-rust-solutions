#[cfg(test)]
mod tests{
    use crate::encoding::{Hex, Base64};
    use std::collections::HashMap;
    use std::fs;

    // set 1 challenge 1: convert hex to base64
    #[test]
    fn set1_challenge1() {
        let s: Hex = Hex::new("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let result: Base64 = s.to_base64();

        assert_eq!(expected, *result);
    }

    // set 1 challenge 2: XOR two hex buffers
    #[test]
    fn set1_challenge2() {
        let buf1: Hex = Hex::new("1c0111001f010100061a024b53535009181c");
        let buf2: Hex = Hex::new("686974207468652062756c6c277320657965");
        let expected: &str = "746865206b696420646f6e277420706c6179";
        let result: Hex = buf1.xor(&buf2);

        assert_eq!(expected, *result);
    }

    fn get_english_letter_freq() -> HashMap<char, f64> {
        [
            ('a', 0.0651738),
            ('b', 0.0124248),
            ('c', 0.0217339),
            ('d', 0.0349835),
            ('e', 0.1041442),
            ('f', 0.0197881),
            ('g', 0.0158610),
            ('h', 0.0492888),
            ('i', 0.0558094),
            ('j', 0.0009033),
            ('k', 0.0050529),
            ('l', 0.0331490),
            ('m', 0.0202124),
            ('n', 0.0564513),
            ('o', 0.0596302),
            ('p', 0.0137645),
            ('q', 0.0008606),
            ('r', 0.0497563),
            ('s', 0.0515760),
            ('t', 0.0729357),
            ('u', 0.0225134),
            ('v', 0.0082903),
            ('w', 0.0171272),
            ('x', 0.0013692),
            ('y', 0.0145984),
            ('z', 0.0007836),
            (' ', 0.1918182)
        ].iter().cloned().collect()
    }

    // encrypt ascii text with a 1-byte key
    fn xor_single_char_cipher(s: &str, k: char) -> String {
        let mut ciphertext: String = String::new();

        for c in s.chars() {
            ciphertext.push(((c as u8) ^ (k as u8)) as char);
        }

        return ciphertext;
    }

    // make a map of character counts
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

    fn make_char_freq_map(len: usize, count_map: &HashMap<char, u32>) -> HashMap<char, f64> {
        count_map
            .iter()
            .map(|(c, n)| (*c, (*n as f64) / (len as f64)))
            .collect()
    }

    // assumption: characters are independent samples from the distribution defined by english_letter_freq
    // the higher the plaintext score, the "more likely" the plaintext is
    fn plaintext_score(english_letter_freq: &HashMap<char, f64>, plaintext: &str) -> f64 {
        let mut score: f64 = 0.0;
        for c in plaintext.chars() {
            // if no character occurred in the plaintext, set frequency difference to 1.0
            // this creates a penalty against plaintexts that don't have letters
            score += english_letter_freq.get(&c).unwrap_or(&0.0);
        }
        return score;
    }

    // set 1 challenge 3: determine 1-byte encryption key
    #[test]
    fn set1_challenge3() {
        let english_letter_freq = get_english_letter_freq();
        let s: Hex = Hex::new("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let s_ascii: String = s.to_ascii();

        let mut best_score: f64 = 0.0;
        let mut best_plaintext: String = "".to_string();

        for k in 0u8..255 {
            let plaintext: String = xor_single_char_cipher(&s_ascii, k as char);
            let score: f64 = plaintext_score(&english_letter_freq, &plaintext.to_ascii_lowercase());

            if score > best_score {
                best_score = score;
                best_plaintext = plaintext;
            }
        }

        assert_eq!("Cooking MC's like a pound of bacon", best_plaintext);
    }

    #[test]
    fn set1_challenge4() {
        let english_letter_freq = get_english_letter_freq();
        let filestr: String =
            fs::read_to_string("4.txt")
            .expect("cannot read 4.txt");

        let mut best_score: f64 = 0.0;
        let mut best_plaintext: String = String::new();
        for line in filestr.lines() {
            let line_ascii: String = Hex::new(line).to_ascii();
            for k in 0u8..255 {
                let plaintext: String = xor_single_char_cipher(&line_ascii, k as char).trim().to_string();
                let score: f64 = plaintext_score(&english_letter_freq, &plaintext.to_ascii_lowercase());

                if score > best_score {
                    best_score = score;
                    best_plaintext = plaintext;
                }
            }
        }

        assert_eq!("Now that the party is jumping", best_plaintext);
    }
}