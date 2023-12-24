use std::collections::HashMap;
use std::path::Path;

// splits (abc, def) -> "abc", "def"
fn split_lr(s: &str) -> (String, String) {
    let (mut l, mut r) = (String::new(), String::new());
    let mut is_left = true;

    for c in s.chars() {
        if c.is_alphabetic() || c.is_numeric() {
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
    let (mut steps, mut ghost_steps) = (0, 0);
    let map_data = std::fs::read_to_string(map_p).expect("Expect map data");
    let (directions, network) = parse_map(&map_data);

    // p1
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

    // p2
    let mut ghost_positions = Vec::<&str>::new();
    for (k, _) in network.iter() {
        if k.ends_with("A") {
            ghost_positions.push(k);
        }
    }
    let mut ghost_cycles = vec![0 as usize; ghost_positions.len()];

    for direction in directions.chars().cycle() {
        for (idx, pos) in ghost_positions.iter().enumerate() {
            if pos.ends_with("Z") && ghost_cycles[idx] == 0 {
                ghost_cycles[idx] = ghost_steps;
            }
        }

        if ghost_cycles.iter().all(|c| *c > 0) {
            break;
        }

        ghost_positions.iter_mut().for_each(|p| {
            if direction == 'L' {
                *p = &network.get(*p).expect("Expect node in network").0;
            } else {
                // R
                *p = &network.get(*p).expect("Expect node in network").1;
            }
        });

        ghost_steps += 1;
        if ghost_steps % 1000000 == 0 {
            println!("ghost steps: {}", ghost_steps);
        }
    }

    (
        steps,
        ghost_cycles
            .iter()
            .fold(1, |acc, c| num::integer::lcm(acc, *c)),
    )
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
        assert_eq!(steps("data/ex2.txt").1, 6)
    }
}
