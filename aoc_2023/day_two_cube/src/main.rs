use std::collections::HashMap;
use std::path::Path;

fn split_game_info(game: &str) -> (usize, &str) {
    let (id_prefix, scores_postfix) = game.split_once(": ").expect("Expect ':' to split gameline");
    let id = id_prefix
        .split_ascii_whitespace()
        .nth(1)
        .expect("Expect str number")
        .parse::<usize>()
        .expect("Expect str to usize id conversion");

    (id, scores_postfix)
}

fn valid_game(game_scores: &str, constraints: &HashMap<&str, usize>) -> bool {
    let set_split = game_scores.split(";").map(|s| s.trim());
    for set in set_split {
        for record in set.split(",").map(|s| s.trim()) {
            let (str_score, color) = record
                .split_once(" ")
                .expect("Expect split color and score");

            let num_score = str_score.parse::<usize>().expect("Expect usize from score");
            if num_score > *constraints.get(color).expect("Expect color in map") {
                return false;
            }
        }
    }

    true
}

fn power(game_scores: &str) -> usize {
    let set_split = game_scores.split(";").map(|s| s.trim());
    let mut max_scores = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    for set in set_split {
        for record in set.split(",").map(|s| s.trim()) {
            let (str_score, color) = record
                .split_once(" ")
                .expect("Expect split color and score");

            let num_score = str_score.parse::<usize>().expect("Expect usize from score");
            max_scores.insert(
                color,
                num_score.max(*max_scores.get(color).expect("Expect color in map")),
            );
        }
    }
    max_scores.values().product()
}

fn day_two_bundle<P: AsRef<Path>>(games_path: P) -> (usize, usize) {
    let mut sum_of_valid = 0;
    let mut sum_of_power = 0;
    let constraints: HashMap<&str, usize> =
        std::collections::HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let games = std::fs::read_to_string(games_path).expect("Expect games filepath for parsing");
    for game in games.lines() {
        let (id, scores) = split_game_info(game);
        // part 1
        if valid_game(&scores, &constraints) {
            sum_of_valid += id;
        }
        // part 2
        sum_of_power += power(scores);
    }
    (sum_of_valid, sum_of_power)
}

fn main() {
    println!(
        "(p1, p2) {:?}",
        day_two_bundle("aoc_2023/day_two_cube/data/input1.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(day_two_bundle("data/example1.txt").0, 8);
    }

    #[test]
    fn ex2() {
        assert_eq!(day_two_bundle("data/example1.txt").1, 2286);
    }
}
