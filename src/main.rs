// encode hex string from bytes
fn hex_encode(bytes: &Vec<u8>) -> String {
    let mut hex_string: String = String::new();
    for &b in bytes.iter() {
        let encoded_char: u8 =
            if b <= 9 {
                b + 48

            } else if 10 <= b && b <= 15 {
                b + 87

            } else {
                panic!("invalid hex-encoded byte with index {}", b)
            }
        ;

        hex_string.push(encoded_char as char);
    }

    return hex_string;
}

// decode hex bytes from a string
fn hex_decode(s: &str) -> Vec<u8> {
    let mut decoded_bytes: Vec<u8> = Vec::new();

    // decode from ASCII
    let s_bytes = s.as_bytes();
    for &val in s_bytes.iter() {
        let decoded_val =
            if 48 <= val && val <= 57 { // 0-9
                val - 48

            } else if 97 <= val && val <= 102 { // a-f
                val - 87

            } else {
                panic!("invalid hex-encoded char: {}", val as char)
            }
        ;

        decoded_bytes.push(decoded_val);
    }

    decoded_bytes
}

// encode base64 string from bytes
fn base64_encode(bytes: &Vec<u8>) -> String {
    let mut b64_string: String = String::new();
    for &b in bytes.iter() {
        // A-Z
        let encoded_char: u8 =
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
                panic!("invalid base64-encoded byte with index {}", b)
            }
        ;
        
        b64_string.push(encoded_char as char);
    }

    return b64_string;
}

// decode base64 bytes from a string
fn base64_decode(s: &str) -> Vec<u8> {
    let mut decoded_bytes: Vec<u8> = Vec::new();

    // decode from ASCII
    let s_bytes: &[u8] = s.as_bytes();
    for &val in s_bytes.iter() {
        let decoded_val: u8 =
            if 65 <= val && val <= 90 { // A-Z
                val - 65

            } else if 97 <= val && val <= 122 { // a-z
                val - 97 + 26

            } else if 48 <= val && val <= 57 {
                val - 48 + 52

            } else if val == 43 {
                62

            } else if val == 47 {
                63

            } else {
                panic!("invalid base64-encoded char: {}", val as char)
            }
        ;

        decoded_bytes.push(decoded_val);
    }

    decoded_bytes
}

// convert hex into base64
fn hex_to_base64(s: &str) -> String {
    // since hex values are 4 bits wide and base64 values are 6 bits wide
    // we can only encode 2 base64 bytes for every 3 hex bytes.
    // thus the total number of hex bytes must be divisible by 3
    if s.len() % 3 != 0 {
        panic!("hex-encoded value cannot be converted to base64: {}", s)
    }

    let decoded_bytes: Vec<u8> = hex_decode(s);

    // encode 2 base64 bytes at a time
    let mut b64_bytes: Vec<u8> = Vec::new();
    for i in 0..(decoded_bytes.len()/3) {
        let b: usize = i*3;

        // first byte
        let b64_byte1 = (decoded_bytes[b] << 2) | ((decoded_bytes[b+1] & 0b00001100) >> 2);

        // second byte
        let b64_byte2: u8 = ((decoded_bytes[b+1] & 0b00000011) << 4) | decoded_bytes[b+2];

        b64_bytes.push(b64_byte1);
        b64_bytes.push(b64_byte2);
    }

    // encode to base64
    let b64_string = base64_encode(&b64_bytes);

    return b64_string
}

// set 1 challenge 1: convert hex to base64
fn set1_challenge1() {
    let s: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let result: String = hex_to_base64(s);

    println!("hex: {0}\nexpected base64: {1}\nreceived base64: {2}", s, expected, result);
    if result == expected {
        println!("set 1 challenge 1 passed");

    } else {
        println!("set 1 challenge 1 failed");
    }
}

// XOR two hex strings
fn hex_xor(buf1: &str, buf2: &str) -> String {
    if buf1.len() != buf2.len() {
        panic!("hex_xor: cannot XOR two hex strings with different lengths");
    }

    let hex1: Vec<u8> = hex_decode(buf1);
    let hex2: Vec<u8> = hex_decode(buf2);

    let mut hex_result: Vec<u8> = Vec::new();
    for i in 0..hex1.len() {
        hex_result.push(hex1[i] ^ hex2[i]);
    }

    return hex_encode(&hex_result);
}

// set 1 challenge 2: XOR two hex buffers
fn set1_challenge2() {
    let buf1: &str = "1c0111001f010100061a024b53535009181c";
    let buf2: &str = "686974207468652062756c6c277320657965";
    let expected: &str = "746865206b696420646f6e277420706c6179";
    let result: String = hex_xor(buf1, buf2);

    println!("buffer 1: {0}\nbuffer 2: {1}\nexpected: {2}\nresulted: {3}",
        buf1, buf2, expected, result);

    if expected == result {
        println!("set 1 challenge 2 passed");

    } else {
        println!("set 1 challenge 2 failed");
    }
}

fn hex_to_ascii(s: &str) -> String {
    if s.len() % 2 != 0 {
        panic!("hex string length must be divisible by 2 to convert to ascii");
    }

    let hex_bytes: Vec<u8> = hex_decode(s);
    let mut ascii_string: String = String::new();
    for i in 0..hex_bytes.len()/2 {
        let b: usize = i * 2;
        let ascii_byte: u8 = (hex_bytes[b] << 4) | hex_bytes[b+1];
        ascii_string.push(ascii_byte as char);
    }

    return ascii_string;
}

fn main() {
    set1_challenge1();
    set1_challenge2();
}
