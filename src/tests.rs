#[cfg(test)]
mod tests{
    use crate::encoding::*;
    use bitvec::prelude::*;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::{self, BufRead};
    use std::path::Path;

    fn read_lines<P>(filename: P) -> io::Result<String>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;

        let mut s = String::new();
        for line in io::BufReader::new(file).lines() {
            if let Ok(ip) = line {
                s.push_str(&ip)
            }
        }
        Ok(s)
    }

    // set 1 challenge 1: convert hex to base64
    #[test]
    fn set1_challenge1() {
        let result = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }

    // set 1 challenge 2: XOR two hex buffers
    #[test]
    fn set1_challenge2() {
        let buf1: String = hex_to_ascii_str("1c0111001f010100061a024b53535009181c");
        let buf2: String = hex_to_ascii_str("686974207468652062756c6c277320657965");
        let expected: &str = "746865206b696420646f6e277420706c6179";
        let xored_bytes: String = ascii_to_hex_str(&xor_bytes(&buf1, &buf2));

        assert_eq!(expected, xored_bytes);
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
    fn xor_single_char(s: &str, k: char) -> String {
        let mut ciphertext: String = String::new();

        for c in s.chars() {
            ciphertext.push(((c as u8) ^ (k as u8)) as char);
        }

        return ciphertext;
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
        let s: String = hex_to_ascii_str("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

        let mut best_score: f64 = 0.0;
        let mut best_plaintext: String = "".to_string();

        for k in 0u8..255 {
            let plaintext: String = xor_single_char(&s, k as char);
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
            let line_ascii: String = hex_to_ascii_str(line);
            for k in 0u8..255 {
                let plaintext: String = xor_single_char(&line_ascii, k as char).trim().to_string();
                let score: f64 = plaintext_score(&english_letter_freq, &plaintext.to_ascii_lowercase());

                if score > best_score {
                    best_score = score;
                    best_plaintext = plaintext;
                }
            }
        }

        assert_eq!("Now that the party is jumping", best_plaintext);
    }

    // encrypt ascii text with a 1-byte key
    fn xor_repeating_key(plaintext: &str, key: &str) -> String {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();
        let mut ciphertext: String = String::new();
        let mut i: usize = 0;

        for c in plaintext.chars() {
            let key_i: usize = i % key_len;
            let key_char: char = key_bytes[key_i] as char;
            ciphertext.push(((c as u8) ^ (key_char as u8)) as char);
            i += 1;
        }

        println!("key: {}; key_len: {};  ciphertext: {}", key, key_len, ciphertext);

        return ciphertext;
    }

    #[test]
    fn set1_challenge5() {
        let line = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let ciphertext=  ascii_to_hex_str(&xor_repeating_key(line, "ICE"));

        assert_eq!(ciphertext, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    }

    fn hamming_distance(str1: &[u8], str2: &[u8]) -> u64 {
        let bits1 = str1.view_bits::<Msb0>();
        let bits2 = str2.view_bits::<Msb0>();

        let mut biter2 = bits2.iter();
        let mut count = 0;
        for b1 in bits1.iter() {
            let xor = *b1 ^ *(biter2.next().expect("hamming_distance: strings have to be same length"));
            count += if xor { 1 } else { 0 };
        }
        return count;
    }

    #[test]
    fn test_hamming() {
        let d = hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes());
        assert_eq!(d, 37);
    }

    // calculate the average Hamming distance given a specific window size
    fn window_hamming_distance(s: &[u8], size: usize) -> f64 {
        let mut i: usize = 0;
        let mut d: u64 = 0;
        while (i+2)*size < s.len() {
            let frame1 = &s[(i*size) .. (i+1)*size];
            let frame2 = &s[(i+1)*size .. (i+2)*size];
            d += hamming_distance(frame1, frame2);
            i += 1;
        }

        return (d as f64) / (s.len() as f64);
    }

    #[test]
    fn set1_challenge6() {
        // load file and decode from base64
        let filestr: String =
            base64_to_ascii_str(
                &read_lines("6.txt")
                .expect("cannot read 6.txt")
            );

        // find keysize
        let filebytes = filestr.as_bytes();
        let mut best_d = window_hamming_distance(filebytes, 1);
        let mut best_key_size: usize = 1;
        for window_size in 2 .. 50 {
            let d = window_hamming_distance(filebytes, window_size);
            if d < best_d {
                best_d = d;
                best_key_size = window_size;
            }
        }

        // arrange blocks to find individual characters of key
        // e.g. if key is length 3, arrange bytes 1, 4, 7, ... into one block;
        // arrange bytes 2, 5, 8, ... into another block; and so on
        let english_letter_freq = get_english_letter_freq();
        let mut blocks = vec![String::new(); best_key_size];
        for i in 0 .. filebytes.len() {
            blocks[i % best_key_size].push(filebytes[i] as char);
        }

        let mut key = String::new();
        for i in 0 .. best_key_size {
            let mut best_score: f64 = 0.0;
            let mut best_char: char = ' ';
            for k in 0u8..255 {
                let plaintext: String = xor_single_char(&blocks[i], k as char);
                let score: f64 = plaintext_score(&english_letter_freq, &plaintext.to_ascii_lowercase());

                if score > best_score {
                    best_score = score;
                    best_char = k as char;
                }
            }

            key.push(best_char as char);
        }

        assert_eq!(key, "Terminator X: Bring the noise");
    }
}