use std::collections::HashSet;
use std::path::Path;

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
    let scorecard_lines: Vec<&str> = scorecards.lines().collect();
    // vec of (#instanced, #matching_card_numbers)
    let mut scratchard_freq_match_tuples: Vec<(usize, usize)> = vec![(1, 0); scorecard_lines.len()];

    for (cur_card_idx, scorecard) in scorecard_lines.into_iter().enumerate() {
        let mut winning_card_count: u32 = 0;

        let scores = scorecard
            .split_once(": ")
            .expect("Expect scorecard split at ': '")
            .1;

        let (current_string, winning_string) = scores
            .split_once(" | ")
            .expect("Expect scores to split at ' | '");

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
        // slide down by magnitude(matching numbers), and increase scratchard count by frequency(cur card)
        scratchard_freq_match_tuples[cur_card_idx].1 += winning_card_count as usize;
        for offset_idx in 1..=scratchard_freq_match_tuples[cur_card_idx].1 as usize {
            scratchard_freq_match_tuples[cur_card_idx + offset_idx].0 +=
                scratchard_freq_match_tuples[cur_card_idx].0;
        }
    }
    (
        score_total_p1,
        scratchard_freq_match_tuples.into_iter().map(|t| t.0).sum(),
    )
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

    #[test]
    fn ex2() {
        assert_eq!(score("data/ex1.txt").1, 30);
    }
}
