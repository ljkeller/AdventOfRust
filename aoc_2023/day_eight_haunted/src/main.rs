use std::collections::HashMap;
use std::path::Path;

// splits (abc, def) -> "abc", "def"
fn split_lr(s: &str) -> (String, String) {
    let (mut l, mut r) = (String::new(), String::new());
    let mut is_left = true;

    for c in s.chars() {
        if c.is_alphabetic() {
            if is_left {
                l.push(c);
            } else {
                r.push(c);
            }
        } else if c == ',' {
            is_left = false;
        }
    }

    (l, r)
}

// splits (abc, def) -> "abc", "def"
// fn split_lr(s: &str) -> (String, String) {
//     let cleaned = s.trim_matches(|c: char| c.is_whitespace() || c == '(' || c == ')');
//     println!("Cleaned: {}", cleaned);
//     let (l, r) = cleaned.split_once(",").expect("Expect split at ','");
//     (l.to_string(), r.to_string())
// }

fn parse_map(data: &str) -> (String, HashMap<String, (String, String)>) {
    let mut network = HashMap::<String, (String, String)>::new();
    let mut data_as_lines = data.lines();

    let directions = data_as_lines.next().unwrap().to_string();

    // space between direction and nodes data
    data_as_lines.next();

    for node in data_as_lines {
        let (entry, exits) = node.split_once(" = ").expect("expect split at ' = '");
        let (l, r) = split_lr(exits);
        network.insert(entry.to_string(), (l, r));
    }

    (directions, network)
}

fn steps<P: AsRef<Path>>(map_p: P) -> (usize, usize) {
    let mut steps = 0;
    let map_data = std::fs::read_to_string(map_p).expect("Expect map data");
    let (directions, network) = parse_map(&map_data);

    let mut cur = "AAA";
    for direction in directions.chars().cycle() {
        if cur == "ZZZ" {
            break;
        }

        if direction == 'L' {
            cur = &network.get(cur).expect("Expect node in map").0;
        } else {
            // R
            cur = &network.get(cur).expect("Expect node in map").1;
        }

        steps += 1;
    }

    (steps, 0)
}

fn main() {
    println!(
        "(p1, p1) {:?}",
        steps("aoc_2023/day_eight_haunted/data/sample1.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(steps("data/ex1.txt").0, 2)
    }

    #[test]
    fn ex_p2() {
        assert_eq!(steps("data/ex1.txt").1, 0)
    }
}
