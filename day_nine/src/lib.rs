use std::{collections::HashSet, io::BufRead};
struct Rope {
    cur_position: (i32, i32),
    last_position: (i32, i32),
}

struct Vector {
    direction: char,
    magnitude: i32,
}

fn step_tail(head: &Rope, tail: &mut Rope, visited: &mut HashSet<(i32, i32)>) {
    let dist_x_squared = (head.cur_position.0 as f32 - tail.cur_position.0 as f32).powi(2);
    let dist_y_squared = (head.cur_position.1 as f32 - tail.cur_position.1 as f32).powi(2);
    let dist_head_tail = (dist_x_squared + dist_y_squared).sqrt();

    if dist_head_tail >= 2.0 {
        tail.last_position = tail.cur_position;
        tail.cur_position = head.last_position;
        visited.insert(tail.cur_position);
    }
}

fn step_head(head: &mut Rope, transform_head: &Vector) {
    head.last_position = head.cur_position;
    match transform_head.direction {
        'U' => {
            head.cur_position.1 += 1;
        }
        'D' => {
            head.cur_position.1 -= 1;
        }
        'L' => {
            head.cur_position.0 -= 1;
        }
        'R' => {
            head.cur_position.0 += 1;
        }
        other => print!("Found invalid direction"),
    }
}

pub fn visited_positions(simulation_fp: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut head = Rope {
        cur_position: (0, 0),
        last_position: (0, 0),
    };
    let mut tail = Rope {
        cur_position: (0, 0),
        last_position: (0, 0),
    };

    if let Ok(simulation_file) = std::fs::File::open(simulation_fp) {
        let transformation_lines = std::io::BufReader::new(simulation_file).lines();

        for transform in transformation_lines {
            let transform_pre_process = transform
                .as_ref()
                .unwrap()
                .split_once(" ")
                .expect("Expect each line to have middle space");
            let transform_head = Vector {
                direction: transform_pre_process.0.chars().next().unwrap(),
                magnitude: transform_pre_process.1.parse::<i32>().unwrap(),
            };
            for _ in 1..=transform_head.magnitude {
                step_head(&mut head, &transform_head);
                step_tail(&head, &mut tail, &mut visited);
            }
        }
    } else {
        println!(
            "Could not find file {} at path {}",
            simulation_fp,
            std::env::current_dir().unwrap().display()
        );
    }

    return visited.len();
}

pub fn temp_two(simulation_fp: &str) -> i32 {
    0
}
