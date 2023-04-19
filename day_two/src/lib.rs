use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_encrypted(encrypted: char) -> Shape {
        let decrypt = HashMap::from([
            ('A', Shape::Rock),
            ('B', Shape::Paper),
            ('C', Shape::Scissors),
            ('X', Shape::Rock),
            ('Y', Shape::Paper),
            ('Z', Shape::Scissors),
        ]);
        return decrypt
            .get(&encrypted)
            .expect("Given invalid encryption data")
            .clone();
    }
}

#[derive(Clone)]
enum Strategy {
    Lose,
    Draw,
    Win
}

impl Strategy {
    fn from_encrypted(encrypted: &char) -> Strategy {
        let decrypt = HashMap::from([
            ('X', Strategy::Lose),
            ('Y', Strategy::Draw),
            ('Z', Strategy::Win)
        ]);

        return decrypt.get(encrypted).expect("Given invalid encryption data").clone();
    }

    /// find the ideal shape for me knowing the opponents shape and what strategy the 
    /// elf says I should follow
    fn counter_opponent(op_shape: &Shape, strat: &Strategy) -> Shape {
        // match
        match op_shape {
            Shape::Rock => {
                match strat {
                    Strategy::Draw => {
                        return Shape::Rock;
                    },
                    Strategy::Lose => {
                        return Shape::Scissors;
                    },
                    Strategy::Win => {
                        return Shape::Paper;
                    }
                }
            },
            Shape::Paper => {
                match strat {
                    Strategy::Draw => {
                        return Shape::Paper;
                    },
                    Strategy::Lose => {
                        return Shape::Rock;
                    },
                    Strategy::Win => {
                        return Shape::Scissors;
                    }
                }
            },
            Shape::Scissors => {
                match strat {
                    Strategy::Draw => {
                        return Shape::Scissors;
                    },
                    Strategy::Lose => {
                        return Shape::Paper;
                    },
                    Strategy::Win => {
                        return Shape::Rock;
                    }
                }
            }
        }
    }
}

struct MyGame {
    op_shape: Shape,
    my_shape: Shape,
}

impl MyGame {
    fn points_from_my_shape(&self) -> i8 {
        let choice_to_points =
            HashMap::from([(Shape::Rock, 1), (Shape::Paper, 2), (Shape::Scissors, 3)]);

        return choice_to_points
            .get(&self.my_shape)
            .copied()
            .expect("My move was invalid");
    }

    fn total_points_gained(&self) -> i8 {
        let mut sum = 0;
        sum += self.points_from_my_shape();

        if self.op_shape == self.my_shape {
            sum += 3;
        } else if self.op_shape == Shape::Rock && self.my_shape == Shape::Paper
            || self.op_shape == Shape::Paper && self.my_shape == Shape::Scissors
            || self.op_shape == Shape::Scissors && self.my_shape == Shape::Rock
        {
            sum += 6;
        }

        return sum;
    }
}

pub fn strategy_one(strategies: &str) -> i64 {
    let mut cum_sum: i64 = 0;

    if let Ok(strategy_file) = File::open(strategies) {
        let games = BufReader::new(strategy_file).lines();
        for game_breakdown in games {
            if let Ok(strategy) = game_breakdown {
                // each line in format:
                // char1 char2
                // example:
                // A X
                 
                let moves: Vec<char> = strategy.chars().collect();
                let game = MyGame {
                    op_shape: Shape::from_encrypted(moves[0]),
                    my_shape: Shape::from_encrypted(moves[2]),
                };

                cum_sum += game.total_points_gained() as i64;
            }
        }
    } else {
        println!(
            "No file found at path: {}",
            current_dir().unwrap().display()
        );
    }

    return cum_sum;
}

pub fn strategy_two(strat_filepath: &str) -> i64 {
    let mut cum_sum = 0;

    if let Ok(strategy_file) = File::open(strat_filepath) {
        let strats = BufReader::new(strategy_file).lines();

        for strategy in strats {
            if let Ok(line) = strategy {
                let game_state: Vec<char> = line.chars().collect();
                let game = MyGame {
                    op_shape: Shape::from_encrypted(game_state[0]),
                    my_shape: Strategy::counter_opponent(&Shape::from_encrypted(game_state[0]), &Strategy::from_encrypted(&game_state[2]))
                };

                cum_sum += game.total_points_gained() as i64;
            }
        }

    } else {
        println!("No file found at path: {}", current_dir().unwrap().display())
    }

    return cum_sum
}
