use std::{ops::Deref, fmt};

const BASE64_PADDING_VAL: u8 = 255;

// XOR two byte vectors
pub fn xor_bytes(buf1: &str, buf2: &str) -> String {
    if buf1.len() != buf2.len() {
        panic!("xor_bytes: cannot XOR two byte vectors with different lengths");
    }

    let bytes1 = buf1.as_bytes();
    let bytes2 = buf2.as_bytes();
    let mut result: String = String::new();
    for i in 0..buf1.len() {
        result.push((bytes1[i] ^ bytes2[i]) as char);
    }

    return result;
}

pub fn hex_char(b: u8) -> char {
    let val =
        if b <= 9 {
            b + 48

        } else if 10 <= b && b <= 15 {
            b + 87

        } else {
            panic!("invalid hex-encoded byte {}", b);
        };
    val as char
}

pub fn hex_val(c: char) -> u8 {
    let c_ascii = c as u8;
    return if 48 <= c_ascii && c_ascii <= 57 {
        c_ascii - 48

    } else if 97 <= c_ascii && c_ascii <= 102 {
        c_ascii - 97 + 10

    } else {
        panic!("invalid hex-encoded byte {}", c);
    }
}

pub fn hex_to_ascii_str(str: &str) -> String {
    if str.len() % 2 != 0 {
        panic!("hex string length must be divisible by 2 to convert to ascii");
    }

    let str_bytes = str.as_bytes();
    let mut ascii_string: String = String::new();
    for i in 0..str_bytes.len()/2 {
        let b: usize = i * 2;
        let hex_val1 = hex_val(str_bytes[b] as char);
        let hex_val2 = hex_val(str_bytes[b+1] as char);
        let ascii_byte: u8 = ((hex_val1 << 4) & 0b11110000) | hex_val2;
        ascii_string.push(ascii_byte as char);
    }

    return ascii_string;
}

pub fn ascii_to_hex_str(str: &str) -> String {
    let str_bytes = str.as_bytes();
    let mut hex_string: String = String::new();
    for i in 0..str_bytes.len() {
        let hex_char_upper: char = hex_char((str_bytes[i] & 0b11110000) >> 4);
        let hex_char_lower: char = hex_char(str_bytes[i] & 0b00001111);
        hex_string.push(hex_char_upper);
        hex_string.push(hex_char_lower);
    }

    return hex_string;
}

pub fn base64_char(b: u8) -> char {
    let val =
        if b <= 25 { // A-Z
            b + 65

        } else if 26 <= b && b <= 51 { // a-z
            b + 71

        } else if 52 <= b && b <= 61 { // 0-9
            b - 4 

        } else if b == 62 { // +
            43

        } else if b == 63 { // /
            47

        } else {
            panic!("invalid base64-encoded byte {}", b)
        };
    val as char
}

pub fn base64_val(c: char) -> u8 {
    let c_ascii = c as u8;
    let val =
        if 65 <= c_ascii && c_ascii <= 90 { // A-Z
            c_ascii - 65

        } else if 97 <= c_ascii && c_ascii <= 122 { // a-z
            c_ascii - 97 + 26

        } else if 48 <= c_ascii && c_ascii <= 57 { // 0-9
            c_ascii - 48 + 52

        } else if c_ascii == 43 { // +
            62

        } else if c_ascii == 47 { // /
            63

        } else if c_ascii == 61 { // = (padding)
            BASE64_PADDING_VAL

        } else {
            panic!("invalid base64-encoded byte {}", c_ascii)
        };
    return val
}

pub fn hex_to_base64(str: &str) -> String {
    // since hex values are 4 bits wide and base64 values are 6 bits wide
    // we can only encode 2 base64 bytes for every 3 hex bytes.
    // thus the total number of hex bytes must be divisible by 3
    if str.len() % 3 != 0 {
        panic!("hex-encoded value cannot be converted to base64: {}", str);
    }

    let str_bytes = str.as_bytes();

    // encode 2 base64 bytes at a time
    let mut b64_str: String = String::new();
    for i in 0..(str_bytes.len()/3) {
        let b: usize = i*3;

        let hex_val1 = hex_val(str_bytes[b] as char);
        let hex_val2 = hex_val(str_bytes[b+1] as char);
        let hex_val3 = hex_val(str_bytes[b+2] as char);

        // first char
        let b64_char1 = base64_char((hex_val1 << 2) | ((hex_val2 & 0b00001100) >> 2));

        // second char
        let b64_char2 = base64_char(((hex_val2 & 0b00000011) << 4) | hex_val3);

        b64_str.push(b64_char1);
        b64_str.push(b64_char2);
    }

    // encode to base64
    return b64_str;

}

pub fn base64_to_ascii_str(str: &str) -> String {
    if str.len() % 4 != 0 {
        panic!("base64 string length must be divisible by 4 to convert to ascii");
    }

    let str_bytes = str.as_bytes();
    let mut ascii_string: String = String::new();
    for i in 0..str_bytes.len()/4 {
        let b: usize = i * 4;
        let val1 = base64_val(str_bytes[b] as char);
        let val2 = base64_val(str_bytes[b+1] as char);
        let val3 = base64_val(str_bytes[b+2] as char);
        let val4 = base64_val(str_bytes[b+3] as char);

        let byte1: u8 = ((val1 << 2) & 0b11111100) | ((val2 >> 4) & 0b00000011);
        let byte2: u8 = ((val2 << 4) & 0b11110000) | ((val3 >> 2) & 0b00001111);
        let byte3: u8 = ((val3 << 6) & 0b11000000) | (val4  & 0b00111111);

        ascii_string.push(byte1 as char);

        if val4 == BASE64_PADDING_VAL {
            // one '=' padding, only decode two bytes
            if val3 != BASE64_PADDING_VAL {
                let byte2: u8 = ((val2 << 4) & 0b11110000) | ((val3 >> 2) & 0b00001111);
                ascii_string.push(byte2 as char);
            }

        } else { // no '=' padding, decode all three possible bytes
            ascii_string.push(byte2 as char);
            ascii_string.push(byte3 as char);
        }
    }

    return ascii_string;
}
