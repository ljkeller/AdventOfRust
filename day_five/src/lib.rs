use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::env::current_dir;

struct SupplyStack {
    stacks: Vec<VecDeque<char>>,
}

impl SupplyStack {
    fn build_stack(contents: &Vec<&str>) -> SupplyStack {
        let mut stack = SupplyStack { stacks: Vec::new() };

        // build stack from the bottom up
        // Assumes each stack has atleast 1 element in it
        contents[..contents.len() - 1]
            .into_iter()
            .rev()
            .for_each(|line| {
                let mut crate_idx = 0;
                // note that chars occur starting at idx 1 and end at n-2
                // note that chars are every 4th element from starting char
                line[1..line.len() - 1]
                    .chars()
                    .step_by(4)
                    .for_each(|val_in_crate| {
                        stack.stack_val(val_in_crate, crate_idx);
                        crate_idx += 1;
                    });
            });
        stack
    }

    // help build supply stack by pushing item into target
    fn stack_val(&mut self, val: char, stack_target: usize) {
        if !val.is_alphabetic() {
            return;
        }

        if stack_target < self.stacks.len() {
            self.stacks[stack_target].push_back(val);
        } else if stack_target == self.stacks.len() {
            self.stacks.push(VecDeque::from([val]));
        } else {
            panic!("Got an out of bounds stack target");
        }
    }

    // Create compressed SupplyStack (just top elements of stacks)
    fn tops(&self) -> Vec<char> {
        self.stacks
            .clone()
            .into_iter()
            .map(|v| v.back().unwrap_or(&' ').clone())
            .collect()
    }
}

struct SupplyProcessor {
    stack: SupplyStack,
}

impl SupplyProcessor {
    fn swap_incrementally(&mut self, num_ele: usize, start_pos: usize, end_pos: usize) {
        let start_stack = self.stack.stacks[start_pos - 1].borrow_mut();
        let top_n = start_stack
            .split_off(start_stack.len() - num_ele)
            .into_iter()
            .rev();

        self.stack.stacks[end_pos - 1].extend::<VecDeque<char>>(FromIterator::from_iter(top_n));
    }
    
    fn swap_chunk(&mut self, num_ele: usize, start_pos: usize, end_pos: usize) {
        let start_stack = self.stack.stacks[start_pos - 1].borrow_mut();
        let top_n = start_stack
            .split_off(start_stack.len() - num_ele);

        self.stack.stacks[end_pos - 1].extend::<VecDeque<char>>(FromIterator::from_iter(top_n));
    }

    fn execute(&mut self, instructions: &Vec<&str>, is_incremental_moving: bool) {
        instructions.into_iter().for_each(|instruction| {
            let mut decoded = instruction.split_whitespace();
            let (num_ele, start, end) = (
                decoded
                    .nth(1)
                    .expect("expect num eles in instruction")
                    .parse::<usize>()
                    .unwrap(),
                decoded
                    .nth(1)
                    .expect("expect start point in instruction")
                    .parse::<usize>()
                    .unwrap(),
                decoded
                    .nth(1)
                    .expect("expect end point in instruction")
                    .parse::<usize>()
                    .unwrap(),
            );
            if is_incremental_moving {
                self.swap_incrementally(num_ele, start, end);
            } else {
                self.swap_chunk(num_ele, start, end);
            }
        });
    }

    fn get_tops(&self) -> Vec<char> {
        self.stack.tops()
    }
}

pub fn supplies_one(stacks_fp: &str) -> String {
    let mut tops_string = String::new();

    if let Ok(file_contents) = std::fs::read_to_string(stacks_fp) {
        let (stacks, instructions): (Vec<&str>, Vec<&str>) = file_contents
            .split_once("\n\n")
            .map(|(a, b)| (a.split("\n").collect(), b.split("\n").collect()))
            .expect("Expect stacks \n\n instructions");

        let supplies = SupplyStack::build_stack(&stacks);

        let mut processor = SupplyProcessor { stack: supplies };
        processor.execute(&instructions, true);

        let top_vector = processor.get_tops();
        tops_string = top_vector.into_iter().collect();
    } else {
        println!(
            "Couldnt find file {} at path {}",
            stacks_fp,
            current_dir().unwrap().display()
        );
    }

    return tops_string;
}

pub fn supplies_two(stacks_fp: &str) -> String {
    let mut tops_string = String::new();

    if let Ok(file_contents) = std::fs::read_to_string(stacks_fp) {
        let (stacks, instructions): (Vec<&str>, Vec<&str>) = file_contents
            .split_once("\n\n")
            .map(|(a, b)| (a.split("\n").collect(), b.split("\n").collect()))
            .expect("Expect stacks \n\n instructions");

        let supplies = SupplyStack::build_stack(&stacks);

        let mut processor = SupplyProcessor { stack: supplies };
        processor.execute(&instructions, false);

        let top_vector = processor.get_tops();
        tops_string = top_vector.into_iter().collect();
    } else {
        println!(
            "Couldnt find file {} at path {}",
            stacks_fp,
            current_dir().unwrap().display()
        );
    }

    return tops_string;
}

// parse format
// first C num lines, take the 2nd char and every 4th char after
// Once you hit a number and not a char, you know the stacks are over (can map that # to the stack if you want)
// then parse move commands
