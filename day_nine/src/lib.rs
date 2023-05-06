use std::{collections::HashSet, io::BufRead, process::Termination};
// start at origin
// track visited set
// perform one step at a time for head
// recalculate tail position
struct Rope {
    cur_position: (i32, i32),
    last_position: (i32, i32)
}

struct Vector{direction: char, magnitude: i32}

pub fn visited_positions(simulation_fp: &str) -> i32 {
    let mut visited : HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut head = Rope{ cur_position: (0,0), last_position: (0,0) };
    let mut tail = Rope {cur_position: (0, 0), last_position: (0, 0) };

    if let Ok(simulation_file) = std::fs::File::open(simulation_fp) {
        let transformation_lines = std::io::BufReader::new(simulation_file).lines();

        for transform in transformation_lines {
            let transform_pre_process = transform.as_ref().unwrap().split_once(" ").expect("Expect each line to have middle space");
            let transform_head = Vector {
                direction: transform_pre_process.0.chars().next().unwrap(),
                magnitude: transform_pre_process.1.parse::<i32>().unwrap()
            };
            for step in 1..=transform_head.magnitude {
                match transform_head.direction {
                    'U' => print!("Up"),
                    'D' => print!("Down"),
                    'L' => print!("Left"),
                    'R' => print!("Right"),
                    other => print!("Found invalid direction")
                }
                // step head
                // step tail
                // update visited
            }
            println!("");

        }
    } else {
        println!("Could not find file {} at path {}", simulation_fp, std::env::current_dir().unwrap().display());
    }

    0
}

pub fn temp_two(simulation_fp: &str) -> i32 {
    0
}
