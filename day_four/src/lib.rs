use std::{fs::File, io::{BufRead, BufReader}};

// Take ranges in form "a-b" into (a, b) inclusive
fn range_str_to_tuple(range: &str) -> (i8, i8) {
    let mut vals = range.split("-");
    (vals.next().expect("Expect val 1").parse::<i8>().unwrap(), vals.next().expect("Expect val 2").parse::<i8>().unwrap())
}

// Takes line in form "a-b,c-d" and returns ("a-b", "c-d")
fn intervals_line_split(line: &String) -> (&str, &str) {
    let mut vals = line.split(",");
    (vals.next().expect("Expect val 1"), vals.next().expect("Expect val 2"))
}

pub fn interval_one(fp: &str) -> i32 {
    let mut fully_overlapping_intervals = 0;

    if let Ok(intervals_file ) = File::open(fp) {
        let interval_lines = BufReader::new(intervals_file).lines();

        for interval_line in interval_lines {
            let line = &interval_line.unwrap();
            let (r1, r2) = intervals_line_split(line);
            let (i1, i2) = (range_str_to_tuple(r1), range_str_to_tuple(r2));
            
            let i1_in_i2 = i2.0 <= i1.0 && i1.1 <= i2.1;
            let i2_in_i1 = i1.0 <= i2.0 && i2.1 <= i1.1;

            fully_overlapping_intervals = if i1_in_i2 || i2_in_i1 {fully_overlapping_intervals + 1} else {fully_overlapping_intervals};
        }
    } else {
        println!("Couldnt fine file at path: {}", std::env::current_dir().unwrap().display());
    }

    return fully_overlapping_intervals;
}

pub fn interval_one_missunderstood(fp: &str) -> i32 {
    let mut fully_overlapping_intervals = 0;

    if let Ok(intervals_file ) = File::open(fp) {
        let interval_lines = BufReader::new(intervals_file).lines();

        let mut intervals = Vec::new();

        for interval_line in interval_lines {
            let line = &interval_line.unwrap();
            let (r1, r2) = intervals_line_split(line);
            let (i1, i2) = (range_str_to_tuple(r1), range_str_to_tuple(r2));
            
            intervals.push(i1);
            intervals.push(i2);
        }
        intervals.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let (mut back, mut front): (usize, usize) = (0, 0);
        loop {
            if back == intervals.len() - 1 { break; }

            let (interval_start, interval_end) = intervals.get(back).expect("Expect tuple in vec");

            let (mut neighbor_start, mut neighbor_end) = intervals.get(front).expect("Expect neighbor tuple exists").clone();
            while front < intervals.len() && interval_start <= &neighbor_start && &neighbor_start <= interval_end && interval_start <= &neighbor_end && &neighbor_end <= interval_end {
                fully_overlapping_intervals += 1;

                println!("Mine: {:?}", (interval_start, interval_end));
                println!("neighbor: {:?}", (neighbor_end, neighbor_end));

                front += 1;
                if front < intervals.len() {
                    (neighbor_start, neighbor_end) = intervals.get(front).unwrap().clone();
                } else {
                    break;
                }
            }

            (back, front) = (back+1, back+2);
        }
    } else {
        println!("Couldnt fine file at path: {}", std::env::current_dir().unwrap().display());
    }

    return fully_overlapping_intervals;
}

pub fn interval_two(fp: &str) -> i64 {
    let mut fully_overlapping_intervals = 0;

    if let Ok(intervals_file ) = File::open(fp) {
        let interval_lines = BufReader::new(intervals_file).lines();

        for interval_line in interval_lines {
            let line = &interval_line.unwrap();
            let (r1, r2) = intervals_line_split(line);
            let (i1, i2) = (range_str_to_tuple(r1), range_str_to_tuple(r2));
            
            let i1_start_overlap_i2 = i2.0 <= i1.0 && i1.0 <= i2.1;
            let i2_start_overlap_i1 = i1.0 <= i2.0 && i2.0 <= i1.1;

            fully_overlapping_intervals = if i1_start_overlap_i2 || i2_start_overlap_i1 {fully_overlapping_intervals + 1} else {fully_overlapping_intervals};
        }
    } else {
        println!("Couldnt fine file at path: {}", std::env::current_dir().unwrap().display());
    }

    return fully_overlapping_intervals;
}