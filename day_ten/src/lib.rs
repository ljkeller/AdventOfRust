use std::{collections::{VecDeque, vec_deque}, io::BufRead, thread::panicking};

struct CPU {
    cycle_count : i32,
    adding : bool,
    add_x : Option<i32>,
    reg1 : i32,
    keep_alive : bool,
    instructions : VecDeque<String>,
    targets : Vec<i32>, 
    sig : usize
}

impl CPU {
    fn new(instruction_set : VecDeque<String>, target_cycles : Vec<i32>) -> Self {
        Self {
            cycle_count : 1,
            adding : false,
            add_x : Option::None,
            reg1 : 1,
            keep_alive : true,
            instructions : instruction_set,
            targets : target_cycles,
            sig : 0
        }
    }

    fn cycle(&mut self) {
        if self.instructions.is_empty() {
            self.keep_alive = false;
            return
        }
        let skip_fetch = self.adding;
        
        // We check signal strength in the "middle" of the cycle
        self.check_signal_strength();
        
        if self.adding {
            self.finish_add_op();
        }

        if !skip_fetch {
            self.fetch_decode_execute();
        }
        
        self.cycle_count += 1;
    }

    fn finish_add_op(&mut self) {
        self.reg1 += self.add_x.expect("Expect value if adding");
        self.adding = false;
        self.add_x = Option::None;
    }

    fn check_signal_strength(&mut self) {
        if self.targets.contains(&self.cycle_count) {
            self.sig += self.reg1 as usize * self.cycle_count as usize;
        }
    }

    fn fetch_decode_execute(&mut self) {
        let op = self.instructions.pop_front().unwrap();
        if op.contains("noop") {
            // no-op
        } else if op.contains("addx"){
            let (_, val) = op.split_once(" ").expect("Expect two vals in addx op");

            self.adding = true;
            self.add_x = Some(val.parse::<i32>().expect("Expect add op to be number"));
        } else {
            panic!("Found bad operation: {op}");
        }
    }
}

pub fn find_signal_strengths(path: &str, targets: Vec<i32>) -> usize {
    let op_file = std::fs::File::open(path).expect("Cant read given file");
    let lines = std::io::BufReader::new(op_file).lines().into_iter();
    let instruction_deque: VecDeque<String> = VecDeque::from_iter(lines.map(|x| x.unwrap()));

    let mut cpu = CPU::new(instruction_deque, targets);
    while cpu.keep_alive {
        cpu.cycle();
    }

    cpu.sig
}

pub fn s2(path: &str) -> usize {
    2+3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_larger_sample() {
        let result = find_signal_strengths("/Users/lucaskeller/code/rust/AdventOfRust/data/sample_day_ten.txt", vec![20, 60, 100, 140, 180, 220]);
        assert_eq!(result, 13140);
    }
}
