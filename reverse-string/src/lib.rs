extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    if input.len() <= 0 {
        return String::from("");
    }

    let mut i = input.len() as i32 - 1;
    let mut output = String::with_capacity(input.len());

    while i >= 0 {
        let char = input.graphemes(true).as_str().chars().nth(i as usize);

        match char {
            Some(char) => output.push(char),
            None => ()
        }
        
        i -= 1;
    }

    output
}
