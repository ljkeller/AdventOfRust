use std::collections::{VecDeque, HashSet};
 
fn is_unique_elements_buffer_of_size(buf: &VecDeque<char>, ideal_size: usize) -> bool {
    HashSet::<char>::from_iter(buf.iter().cloned()).len() == buf.len() && buf.len() == ideal_size
}

pub fn subroutine_one(comstream_fp: &str) -> i32 {
    let mut chars_until_marker = 0;
    if let Ok(comstream) = std::fs::read_to_string(comstream_fp) {
        let mut last_window = VecDeque::new();
        let comstream_chars = comstream.chars().collect::<Vec<char>>();

        for c in comstream_chars {
            last_window.push_front(c);
            chars_until_marker += 1;

            if last_window.len() > 4 { last_window.pop_back(); }

            if is_unique_elements_buffer_of_size(&last_window, 4) { break }
        }
    } else {
        println!("Unable to find fp {} at path {}", comstream_fp, std::env::current_dir().unwrap().display())
    }

    return chars_until_marker;
}

pub fn subroutine_two(comstream_fp: &str) -> i32 {
    let mut chars_until_marker = 0;
    if let Ok(comstream) = std::fs::read_to_string(comstream_fp) {
        let mut last_window = VecDeque::new();
        for c in comstream.chars() {
            last_window.push_front(c);
            chars_until_marker += 1;

            if last_window.len() > 14 { last_window.pop_back(); }

            if is_unique_elements_buffer_of_size(&last_window, 14) { break; }
        }

    } else {
        println!("Unable to find fp {} at path {}", comstream_fp, std::env::current_dir().unwrap().display())
    }
    return chars_until_marker;
}