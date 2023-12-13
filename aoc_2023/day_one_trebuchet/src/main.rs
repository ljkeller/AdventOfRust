use std::path::Path;

fn sum_of_calibration_values<P: AsRef<Path>>(path: P) -> usize {
    let mut sum: usize = 0;
    let string_contents = std::fs::read_to_string(path).unwrap();
    let lines = string_contents.lines();
    for line in lines {
        let mut first_digit : Option<u8> = None;
        let mut last_digit: Option<u8> = None;
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
    }
    sum
}

fn main() {
    println!("2023 day 1 part 1: {}", sum_of_calibration_values("/home/lucaskeller/code/rust/aoc_2022/2023_data/day_1_input_1.txt"))
}

#[cfg(test)]
mod tests {
    use crate::sum_of_calibration_values;

    #[test]
    fn sum_example() {
        let given_sum = 142;
        assert_eq!(given_sum, sum_of_calibration_values("/home/lucaskeller/code/rust/aoc_2022/2023_data/day_1_example_1.txt"));
    }
}