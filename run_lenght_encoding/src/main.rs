// Good morning! Here's your coding interview problem for today.

// This problem was asked by Amazon.

// Run-length encoding is a fast and simple method of encoding strings. The basic idea is to represent repeated successive characters as a single count and character. For example, the string "AAAABBBCCDAA" would be encoded as "4A3B2C1D2A".

// Implement run-length encoding and decoding. You can assume the string to be encoded have no digits and consists solely of alphabetic characters. You can assume the string to be decoded is valid.


fn main() {
    let input_string = String::from("AAABAAAAAAAAAAAAABBBBCEE");
    let encoded_string = encode(&input_string);

    println!("  Input string: {}", &input_string);
    println!("Encoded string: {}", encode(&input_string));
    println!("Decoded string: {}", decode(&encoded_string));
    
}

fn encode(i_string: &String) -> String {
    let mut encoded_string = String::new();
    let mut cur_char: char = '0'; //initially set to zero as we are promised that the string to be encoded only concists of letters.
    let mut cur_char_count: u32 = 0;
    let string_len = i_string.len();
    for (i, char) in i_string.chars().enumerate() {
        if char == cur_char {
            cur_char_count += 1;
        } else {
            if cur_char != '0' {
                encoded_string += &cur_char_count.to_string();
                encoded_string += &cur_char.to_string();
            }
            cur_char = char;
            cur_char_count = 1;
        }
        if i == string_len - 1 {
            encoded_string += &cur_char_count.to_string();
            encoded_string += &cur_char.to_string();
        }
    }
    encoded_string
}

fn decode(i_string: &String) -> String {
    let mut decoded_string = String::new();
    let mut char_count = String::from("");
    for char in i_string.chars() {
        if char.is_numeric() {
            char_count += &char.to_string();
        } else {
            let num_char_count: u32 = char_count.parse().unwrap();
            for _x in 0..num_char_count {
                decoded_string.push(char);
            }
            char_count = String::from("");
        }
    }
    decoded_string
}
