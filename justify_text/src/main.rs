fn main() {
    const LINE_LENGTH: u32 = 60;
    let input_text: Vec<&str> = "A string of text to split into lines and then justify to some fixed lenght. The text can be e.g. The quick brown fox jumps over the lazy dog.".split(" ").collect();

//    justified(input_text, LINE_LENGTH);
    justified2(input_text, LINE_LENGTH);
}

struct TextLine {
    text_vector: Vec<String>,
    len_single_spaces: u32,
    len_no_spaces: u32,
    word_count: u32
}

impl TextLine {
    fn new() -> TextLine {
        TextLine {
            text_vector: vec!(String::new()),
            len_single_spaces: 0,
            len_no_spaces: 0,
            word_count: 0
        }
    }
}
fn justified2(text_vec: Vec<&str>, line_lenght: u32) {
    let line_len = line_lenght;
    let mut output_text_vec = vec!();
    let mut current_line = TextLine::new();
    for word in text_vec {
        if current_line.len_single_spaces + word.len() as u32 >= line_lenght {
            output_text_vec.push(current_line);
            current_line = TextLine::new();
        }
        current_line.text_vector.push(word.to_string());
        current_line.len_single_spaces += word.len() as u32 + 1;
        current_line.len_no_spaces += word.len() as u32;
        current_line.word_count += 1;
    }
    output_text_vec.push(current_line);
    
    for line in output_text_vec.iter() {
        let mut output_string = String::new();
        if line.word_count == 1 {
            for word in line.text_vector.iter() { //unneccessary iter, should be able to just get value with line.text_vector[0] but that didn't work.
                output_string += word;
            }
        } else {
            let count_spaces = (line_len - line.len_no_spaces) / (line.word_count - 1);
            let mut rem_spaces = (line_len - line.len_no_spaces) % (line.word_count - 1);
            for (i, word) in line.text_vector.iter().enumerate() {
                if i > 1 {
                    for _j in 0..count_spaces {
                        output_string += " ";
                    }
                    if rem_spaces > 0 {
                        output_string += " ";
                        rem_spaces -= 1;
                    }
                }
                output_string += word;
            }
        }
        println!("Start:{}:Stop", output_string);
    }
}

//bad first attempt
// fn justified(input_vector: Vec<&str>, l_length: i32) {
//     let mut current_output_elements: Vec<&str> = Vec::new();
//     let mut rem_chars: i32 = l_length;
//     for word in input_vector.iter().peekable() {
//         if rem_chars - word.chars().count() as i32 <= 0 {
//             let mut output_line = String::new();
//             let quote = rem_chars as usize / (current_output_elements.len() - 1);
//             let mut rem = rem_chars as usize % (current_output_elements.len() - 1);
//             let current_elements_lenght = current_output_elements.len();
//             for (i, l_word) in current_output_elements.iter().enumerate() {
//                 output_line = output_line + l_word;
//                 if i != current_elements_lenght {
//                     output_line = output_line + " ";
//                     if quote > 0 {
//                         for _j in 1..quote {
//                             output_line = output_line + " ";
//                         }
//                     }
//                     if rem > 0 {
//                         output_line = output_line + " ";
//                         rem = rem - 1;
//                     }
//                 }
//             }
//             // println!("{}", quote);
//             println!("{}", output_line);
//             println!("{}", output_line.len());
//             rem_chars = l_length;
//             current_output_elements = Vec::new();
//         }
//         current_output_elements.push(word);
//         rem_chars = rem_chars - word.chars().count() as i32 - 1;
        
//     }
// }
