use std::collections::HashSet;
use std::{borrow::Borrow, path::Path};

fn ints(ints_str: &str) -> Vec<u32> {
    let mut ints = Vec::new();
    let toks = ints_str.split_ascii_whitespace();
    toks.for_each(|t| {
        let p = t.parse::<u32>();
        match p {
            Ok(p) => ints.push(p),
            _ => {}
        }
    });

    ints
}

fn score<P: AsRef<Path>>(score_ref: P) -> (usize, usize) {
    let mut score_total_p1: usize = 0;

    let scorecards = std::fs::read_to_string(score_ref).expect("Expect scorecard file");
    let scorecard_lines = scorecards.lines();

    for scorecard in scorecard_lines {
        //p1
        let mut winning_card_count: u32 = 0;

        let scores = scorecard
            .split_once(": ")
            .expect("Expect scorecard split at ': '")
            .1;

        let (current_string, winning_string) = scores
            .split_once(" | ")
            .expect("Expect scorecard to split at ': '");

        let current_cards = ints(current_string);
        let winning_cards: HashSet<u32> = HashSet::from_iter(ints(winning_string));

        for card in current_cards {
            if winning_cards.contains(&card) {
                winning_card_count += 1;
            }
        }

        if winning_card_count > 0 {
            score_total_p1 += 2_usize.pow(winning_card_count - 1);
        }
    }

    (score_total_p1, 0)
}

fn main() {
    println!(
        "(p1, p2) {:?}",
        score("aoc_2023/day_four_scratchcards/data/sample1.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(score("data/ex1.txt").0, 13);
    }
}
