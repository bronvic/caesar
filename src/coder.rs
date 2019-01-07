pub const ALPHABET_SIZE: u8 = 'z' as u8 - 'a' as u8 + 1;

pub fn encode(data: String, shift: u8) -> String {
    // Create vector of maximum possible size to avoid relocation
    let mut vec: Vec<u8> = Vec::with_capacity(data.len());
    let mut num_char: u8;

    for c in data.chars() {
        num_char = c as u8;

        if c >= 'a' && c <= 'z' {
            num_char += shift;
            if num_char > 'z' as u8 {
                num_char = num_char - ALPHABET_SIZE;
            }
        } else if  c >= 'A' && c <= 'Z' {
            num_char += shift;
            if num_char > 'Z' as u8 {
                num_char = num_char - ALPHABET_SIZE;
            }
        }

        vec.push(num_char);
    }
    vec.push('\n' as u8);

    String::from_utf8(vec.clone()).unwrap()
}

pub fn decode(data: String, shift: u8) -> String {
    encode(data, ALPHABET_SIZE - shift)
}