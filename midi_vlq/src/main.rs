fn main() {
    println!("Hello, world!");
}


fn encode(n: i32) -> Vec<u8> {
    if n == 0 {
        return vec!(0 as u8);
    }
    let mut num = n;
    let mut encoded: Vec<u8> = Vec::new();
    let mut counter = 0;
    while num > 0 {
        let mut lower_7 = num & 0x7f;

        if counter > 0 {
            lower_7 = lower_7 | 0x80;
        }
        counter += 1;
        encoded.push(lower_7 as u8);
        num = num >> 7;
    }
    encoded.reverse();
    encoded
}

fn decode(data: &[u8]) -> i32 {
    let mut accum: i32 = 0;
    for part in data {
        let more_flag: bool = part & 0x80 > 0;
        accum = accum + (part & 0x7F) as i32;
        if more_flag {
            accum = accum << 7;
        } else {
            break;
        }
    }
    accum
}

#[cfg(test)]
mod sample_tests {
    use super::{encode, decode};

    fn do_test_enc(n: i32, expected: &[u8]) {
        let actual = encode(n);
        assert_eq!(actual, expected, "\nencode({n:?}) produced {actual:?}, expected {expected:?}");
    }
    fn do_test_dec(bytes: &[u8], expected: i32) {
        let actual = decode(bytes);
        assert_eq!(actual, expected, "\ndecode({bytes:?}) produced {actual:?}, expected {expected:?}");
    }

    #[test]
    fn single_byte_encoding() {
        do_test_enc(0, &[0x00]);
        do_test_enc(127, &[0x7F]);
        do_test_enc(55, &[0x37]);
        do_test_enc(69, &[0x45]);
    }
    #[test]
    fn single_byte_decoding() {
        do_test_dec(&[0x00], 0);
        do_test_dec(&[0x7F], 127);
        do_test_dec(&[0x37], 55);
        do_test_dec(&[0x45], 69);
        // Extra
        do_test_dec(&[0x00, 0x7F, 0x7F], 0);
        do_test_dec(&[0x7F, 0x81, 0x00], 127);
        do_test_dec(&[0x45, 0x45, 0x45], 69);
        do_test_dec(&[0x37, 0xFF, 0xFF, 0x7F], 55);
    }
    #[test]
    fn two_byte_encoding() {
        do_test_enc(128, &[0x81, 0x00]);
        do_test_enc(16383, &[0xFF, 0x7F]);
        do_test_enc(8192, &[0xC0, 0x00]);
        do_test_enc(6969, &[0xB6, 0x39]);
    }
    #[test]
    fn two_byte_decoding() {
        do_test_dec(&[0x81, 0x00], 128);
        do_test_dec(&[0xFF, 0x7F], 16383);
        do_test_dec(&[0xC0, 0x00], 8192);
        do_test_dec(&[0xB6, 0x39], 6969);
        // Extra
        do_test_dec(&[0x81, 0x00, 0xFF], 128);
        do_test_dec(&[0xFF, 0x7F, 0x00], 16383);
        do_test_dec(&[0xC0, 0x00, 0x81, 0x80, 0x00], 8192);
        do_test_dec(&[0xB6, 0x39, 0xFF, 0xFF], 6969);
    }
    #[test]
    fn four_byte_encoding() {
        do_test_enc(2097152, &[0x81, 0x80, 0x80, 0x00]);
        do_test_enc(268435455, &[0xFF, 0xFF, 0xFF, 0x7F]);
        do_test_enc(134217728, &[0xC0, 0x80, 0x80, 0x00]);
        do_test_enc(69696969, &[0xA1, 0x9D, 0xFB, 0x49]);
    }
    #[test]
    fn four_byte_decoding() {
        do_test_dec(&[0x81, 0x80, 0x80, 0x00], 2097152);
        do_test_dec(&[0xFF, 0xFF, 0xFF, 0x7F], 268435455);
        do_test_dec(&[0xC0, 0x80, 0x80, 0x00], 134217728);
        do_test_dec(&[0xA1, 0x9D, 0xFB, 0x49], 69696969);
        // Extra
        do_test_dec(&[0x81, 0x80, 0x80, 0x00, 0x7F], 2097152);
        do_test_dec(&[0xFF, 0xFF, 0xFF, 0x7F, 0x81, 0x00], 268435455);
        do_test_dec(&[0xC0, 0x80, 0x80, 0x00, 0xFF, 0xFF, 0x7F], 134217728);
        do_test_dec(&[0xA1, 0x9D, 0xFB, 0x49, 0x00, 0x00], 69696969);
    }
}