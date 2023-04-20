use std::fs::File;
use std::collections::{HashSet, HashMap};
use std::env::current_dir;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
struct CompartmentedRucksack {
    sack_one: HashSet<char>,
    sack_two: HashSet<char>
}

impl CompartmentedRucksack {
    fn from_string(contents: &str) -> CompartmentedRucksack {
        let (a, b) = contents.split_at(contents.len()/2);
        let rucksack = CompartmentedRucksack {
            sack_one: HashSet::from_iter(a.chars()),
            sack_two: HashSet::from_iter(b.chars())
        };

        return rucksack;
    }

    fn find_error(&self) -> char {
        return self.sack_one.intersection(&self.sack_two).collect::<Vec<&char>>()[0].clone();
    }

    fn priority(item: &char) -> i8 {
        let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut priorities = HashMap::new();
        for (prio, a) in alpha.chars().enumerate() {
            priorities.insert(a, prio+1);
        }
        return priorities.get(item).expect("Must have char in alphabet").clone() as i8;
    }
}

struct GroupRucksack {
    sack_one: HashSet<char>,
    sack_two: HashSet<char>,
    sack_three: HashSet<char>
}

impl GroupRucksack {
    fn from_strings(group: (&str, &str, &str)) -> GroupRucksack {
        let (a, b, c) = group;

        return GroupRucksack { sack_one: HashSet::from_iter(a.chars()), sack_two: HashSet::from_iter(b.chars()), sack_three: HashSet::from_iter(c.chars()) };
    }

    fn extract_badge(&self) -> char {
        let one_intersect_two: HashSet<char> = self.sack_one.intersection(&self.sack_two).cloned().collect();
        return one_intersect_two.intersection(&self.sack_three).collect::<Vec<&char>>()[0].clone();
    }

    fn priority(item: &char) -> i8 {
        CompartmentedRucksack::priority(item)
    }
}

pub fn rucksack_one(rucksack_fp: &str) -> i64 {
    let mut cum_sum = 0;
    if let Ok(rucksack_file) = File::open(rucksack_fp)
    {
        let rucksack_lines = BufReader::new(rucksack_file).lines();

        for rs_line in rucksack_lines {
            let rucksack = CompartmentedRucksack::from_string(&rs_line.expect("Require line contents for rucksack"));
            cum_sum += CompartmentedRucksack::priority(&rucksack.find_error()) as i64;
        }

    } else {
        println!("Couldnt find file at path: {}", current_dir().unwrap().to_str().unwrap());
    }

    return cum_sum;
}

pub fn rucksack_two(rucksack_fp: &str) -> i64 {
    let mut cum_sum = 0;
    if let Ok(rucksack_file) = File::open(rucksack_fp)
    {
        let elf_lines = BufReader::new(rucksack_file).lines();
        let mut groups = Vec::new();
        for elf in elf_lines {
            groups.push(elf.expect("Must be string in line"));

            if groups.len() == 3 {
                let (a, b, c) = groups.iter().collect_tuple().unwrap();
                cum_sum += GroupRucksack::priority(&GroupRucksack::from_strings((a, b, c)).extract_badge()) as i64;

                groups.clear();
            }
        }

    } else {
        println!("Couldnt find file at path: {}", current_dir().unwrap().to_str().unwrap());
    }

    return cum_sum;
}