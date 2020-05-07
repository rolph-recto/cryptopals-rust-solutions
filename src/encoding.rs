use std::fmt;

pub type Bytes = Vec<u8>;

pub trait Decodable {
    fn decode(&self) -> Bytes;
}

pub struct Hex(String);

impl Hex {
    pub fn new(s: &str) -> Hex {
        Hex(s.to_string())
    }

    pub fn encode(bytes: &Bytes) -> Self {
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

        return Self::new(&hex_string);
    }

    // XOR two hex strings
    pub fn xor(&self, buf2: &Hex) -> Hex {
        if self.0.len() != buf2.0.len() {
            panic!("hex_xor: cannot XOR two hex strings with different lengths");
        }

        let hex1: Vec<u8> = self.decode();
        let hex2: Vec<u8> = buf2.decode();

        let mut hex_result: Vec<u8> = Vec::new();
        for i in 0..hex1.len() {
            hex_result.push(hex1[i] ^ hex2[i]);
        }

        return Self::encode(&hex_result);
    }

    // convert hex into base64
    pub fn to_base64(&self) -> Base64 {
        // since hex values are 4 bits wide and base64 values are 6 bits wide
        // we can only encode 2 base64 bytes for every 3 hex bytes.
        // thus the total number of hex bytes must be divisible by 3
        if self.0.len() % 3 != 0 {
            panic!("hex-encoded value cannot be converted to base64: {}", self.0)
        }

        let decoded_bytes: Vec<u8> = self.decode();

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
        return Base64::encode(&b64_bytes);
    }

    // convert to an ASCII-encoded string
    pub fn to_ascii(&self) -> String {
        if self.0.len() % 2 != 0 {
            panic!("hex string length must be divisible by 2 to convert to ascii");
        }

        let hex_bytes: Vec<u8> = self.decode();
        let mut ascii_string: String = String::new();
        for i in 0..hex_bytes.len()/2 {
            let b: usize = i * 2;
            let ascii_byte: u8 = ((hex_bytes[b] << 4) & 0b11110000) | hex_bytes[b+1];
            ascii_string.push(ascii_byte as char);
        }

        return ascii_string;
    }
}

impl Decodable for Hex {
    // decode hex bytes from a string
    fn decode(&self) -> Bytes {
        let mut decoded_bytes: Bytes = Vec::new();

        // decode from ASCII
        let s_bytes = self.0.as_bytes();
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
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub struct Base64(String);

impl Base64 {
    pub fn new(s: &str) -> Self {
        Base64(s.to_string())
    }

    // encode base64 string from bytes
    pub fn encode(bytes: &Bytes) -> Base64 {
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

        return Self(b64_string);
    }
}

impl Decodable for Base64 {
    // decode base64 bytes from a string
    fn decode(&self) -> Bytes {
        let mut decoded_bytes: Vec<u8> = Vec::new();

        // decode from ASCII
        let s_bytes: &[u8] = self.0.as_bytes();
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
}

impl fmt::Display for Base64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
