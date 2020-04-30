// set 1 challenge 1: convert hex into base64
fn hex_to_base64(s: &String) -> String {
    // since hex values are 4 bits wide and base64 values are 6 bits wide
    // we can only encode 2 base64 bytes for every 3 hex bytes.
    // thus the total number of hex bytes must be divisible by 3
    if s.len() % 3 != 0 {
        panic!("hex-encoded value cannot be converted to base64: {}", s)
    }

    let s_bytes = s.as_bytes();
    let mut decoded_bytes: Vec<u8> = Vec::new();

    // decode from ASCII
    for &val in s_bytes.iter() {
        let decoded_val =
            if 48 <= val && val <= 57 { // 0-9
                val - 48

            } else if 97 <= val && val <= 102 { // a-f
                val - 87

            } else {
                panic!("invalid hex-encoded string: {}", val as char)
            }
        ;

        decoded_bytes.push(decoded_val);
    }

    // encode 2 base64 bytes at a time
    let mut b64_encoded: Vec<u8> = Vec::new();
    for i in 0..(decoded_bytes.len()/3) {
        let b: usize = i*3;

        // first byte
        let b64_byte1 = (decoded_bytes[b] << 2) | ((decoded_bytes[b+1] & 0b00001100) >> 2);

        // second byte
        let b64_byte2: u8 = ((decoded_bytes[b+1] & 0b00000011) << 4) | decoded_bytes[b+2];

        b64_encoded.push(b64_byte1);
        b64_encoded.push(b64_byte2);
    }

    // encode to base64
    let mut b64_string: String = String::new();
    for &b in b64_encoded.iter() {
        // A-Z
        let b64_char: u8 =
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
        
        b64_string.push(b64_char as char);
    }

    return b64_string
}

fn main() {
    // set 1 challenge 1
    let s: String = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    println!("hex: {0}\nbase64: {1}", s, hex_to_base64(&s));
}
