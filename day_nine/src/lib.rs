use std::{collections::HashSet, io::BufRead};

const N_KNOTS : usize = 10;
#[derive(Default, Clone, Copy, Debug)]
struct Rope {
    cur_position: (i32, i32),
    last_position: (i32, i32),
}

struct Vector {
    direction: char,
    magnitude: i32,
}

fn step_pair(head: &Rope, tail: &mut Rope, visited: &mut HashSet<(i32, i32)>, mark_visited: bool) {
    let dist_x = head.cur_position.0 - tail.cur_position.0;
    let dist_y = head.cur_position.1 - tail.cur_position.1;
    let dist_x_squared = (dist_x as f32).powi(2);
    let dist_y_squared = (dist_y as f32).powi(2);

    let dist_head_from_tail = (dist_x_squared + dist_y_squared).sqrt();

    if dist_head_from_tail >= 2.0 {
        tail.last_position = tail.cur_position;

        if dist_x == 0 && dist_y == 2 {
            // up
            tail.cur_position.1 = tail.cur_position.1 + 1;
        } else if dist_x > 0 && dist_y > 0 {
            // up right
            tail.cur_position.0 = tail.cur_position.0 + 1;
            tail.cur_position.1 = tail.cur_position.1 + 1;
        } else if dist_x == 2 && dist_y == 0 {
            // right
            tail.cur_position.0 = tail.cur_position.0 + 1;
        } else if dist_x > 0 && dist_y < 0 {
            // down right
            tail.cur_position.0 = tail.cur_position.0 + 1;
            tail.cur_position.1 = tail.cur_position.1 - 1;
        } else if dist_x < 0 && dist_y > 0 {
            // up left
            tail.cur_position.0 = tail.cur_position.0 - 1;
            tail.cur_position.1 = tail.cur_position.1 + 1;
        } else if dist_x == -2 && dist_y == 0 {
            // left
            tail.cur_position.0 = tail.cur_position.0 - 1;
        } else if dist_x < 0 && dist_y < 0 {
            // down left
            tail.cur_position.0 = tail.cur_position.0 - 1;
            tail.cur_position.1 = tail.cur_position.1 - 1;
        } else if dist_x == 0 && dist_y == -2 {
            // down
            tail.cur_position.1 = tail.cur_position.1 - 1;
        } else {
            println!("Something bad happened with stepping body! Dists: {dist_x}, {dist_y}");
        }

        if mark_visited { visited.insert(tail.cur_position); }
    } 
}

fn step_body_and_tail(head: &Rope, body: &mut [Rope; N_KNOTS-1], visited: &mut HashSet<(i32, i32)>) {
    let mut prev = head;
    for (idx, knot) in body.iter_mut().enumerate() {
        // why N_KNOTS -1 -1 ?
        // -1 because array is 0 indexed
        // -1 because HEAD is seperate from rest of body
        let visiting_last_knot = idx == N_KNOTS - 1 - 1;
        step_pair(prev, knot, visited, visiting_last_knot);
        prev = knot;
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

    let mut head = Rope::default();
    let mut tail = Rope::default();

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
                step_pair(&head, &mut tail, &mut visited, true);
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

pub fn visited_positions2(simulation_fp: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    let mut head = Rope::default();
    let mut body = [Rope::default(); N_KNOTS-1];

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
                step_body_and_tail(&head, &mut body, &mut visited);
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
