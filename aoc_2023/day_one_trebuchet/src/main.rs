use std::io::BufRead;
use std::path::Path;

fn sum_of_calibration_values<P: AsRef<Path>>(path: P) -> usize {
    let mut sum: usize = 0;
    let string_contents = std::fs::read_to_string(path).unwrap();
    let lines = string_contents.lines();
    for line in lines {
        sum += line_sum(line);
    }
    sum
}

fn line_sum(line: &str) -> usize {
    let mut first_digit: Option<u8> = None;
    let mut last_digit: Option<u8> = None;
    let mut sum: usize = 0;
    for char in line.chars() {
        if char.is_numeric() && first_digit.is_none() {
            first_digit = Some(char.to_digit(10).unwrap() as u8);
            last_digit = Some(char.to_digit(10).unwrap() as u8);
        } else if char.is_numeric() {
            last_digit = Some(char.to_digit(10).unwrap() as u8);
        }
    }
    if first_digit.is_some() && last_digit.is_some() {
        sum += (first_digit.unwrap() as usize * 10) + (last_digit.unwrap() as usize);
    }
    sum
}

fn sum_of_calibration_values_2<P: AsRef<Path>>(path: P) -> usize {
    let num_mapping = std::collections::HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let calibration_file = std::fs::File::open(path).expect("Expect calibration file");
    let lines = std::io::BufReader::new(calibration_file).lines();

    let mut sum = 0;
    for line in lines.map(|x| x.expect("Expect valid String in line")) {
        let line_bytes = line.as_bytes();
        if line_bytes.len() == 0 {
            continue;
        }

        let end_of_line = line_bytes.len();
        let mut first_digit: Option<u8> = None;
        let mut last_digit: Option<u8> = None;

        for (byte_idx, byte) in line_bytes.iter().enumerate() {
            if byte.is_ascii_digit() && first_digit.is_none() {
                let char_digit = byte.clone() as char;
                first_digit = Some(char_digit.to_digit(10).unwrap() as u8);
                last_digit = first_digit;
            } else if byte.is_ascii_digit() {
                let char_digit = byte.clone() as char;
                last_digit = Some(char_digit.to_digit(10).unwrap() as u8);
            } else {
                // ascii chars
                for (k, v) in num_mapping.iter() {
                    let key_bytes = k.as_bytes();
                    let mut matching_chars = 0;
                    let mut key_byte_idx = 0;
                    while key_byte_idx + byte_idx < end_of_line
                        && key_byte_idx < k.len()
                        && key_bytes[key_byte_idx] == line_bytes[key_byte_idx + byte_idx]
                    {
                        matching_chars += 1;
                        key_byte_idx += 1;
                    }
                    if matching_chars == k.len() {
                        if first_digit.is_none() {
                            first_digit = Some(v.clone());
                            last_digit = Some(v.clone());
                        } else {
                            last_digit = Some(v.clone());
                        }
                    }
                }
            }
        }
        if first_digit.is_some() && last_digit.is_some() {
            sum += (first_digit.unwrap() as usize * 10) + (last_digit.unwrap() as usize);
        }
    }
    sum
}

fn main() {
    println!(
        "2023 day 1 part 1: {}",
        sum_of_calibration_values("2023_data/day_1_input_1.txt")
    );
    println!(
        "2023 day 1 part 2: {}",
        sum_of_calibration_values_2("2023_data/day_1_input_1.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::sum_of_calibration_values;
    use crate::sum_of_calibration_values_2;

    #[test]
    fn sum_example() {
        let given_sum = 142;
        // Tests are ran from module directory
        assert_eq!(
            given_sum,
            sum_of_calibration_values("../../2023_data/day_1_example_1.txt")
        );
    }

    #[test]
    fn sum_2_example() {
        let given_sum = 281;
        // Tests are ran from module directory
        assert_eq!(
            given_sum,
            sum_of_calibration_values_2("../../2023_data/day_1_example_2.txt")
        )
    }
}
