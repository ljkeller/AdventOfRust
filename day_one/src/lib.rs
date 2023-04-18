use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::env;
use std::collections::BinaryHeap;

pub fn calories_one(list_path: &str) -> i64 {
    let mut max_sum: i64 = 0;
    if let Ok(file) = File::open(list_path) {
        let reader = BufReader::new(file);

        let mut cur: i64 = 0;
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.is_empty() {
                    cur = 0;
                }
                else {
                    cur += line_content.parse::<i64>().unwrap();
                    max_sum = cmp::max(max_sum, cur);
                }
            }
        }

        println!("Max sum: {}",max_sum);
    }
    else {
        println!("no file found in {}", env::current_dir().unwrap().display());
    }
    return max_sum;
}

pub fn calories_two(list_path: &str) -> i64 {
    let mut top3_transformed = BinaryHeap::new();
    if let Ok(file) = File::open(list_path) {
        let reader = BufReader::new(file);

        let mut cur: i64 = 0;
        for line in reader.lines() {
            if let Ok(line_content) = line {
                if line_content.is_empty() {
                    top3_transformed.push(-cur);

                    if top3_transformed.len() > 3 { top3_transformed.pop(); }
                    cur = 0;
                }
                else {
                    cur += line_content.parse::<i64>().unwrap();
                }
            }
        }
    }
    else {
        println!("no file found in {}", env::current_dir().unwrap().display());
    }

    // return sum of top3
    return -top3_transformed.iter().sum::<i64>();
}