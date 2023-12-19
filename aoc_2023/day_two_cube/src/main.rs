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

fn valid_game(sets: &str, constraints: &HashMap<&str, usize>) -> bool {
    let set_split = sets.split(";").map(|s| s.trim());
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

fn sum_of_possible_games<P: AsRef<Path>>(games_path: P) -> usize {
    let mut cum_sum = 0;
    let constraints: HashMap<&str, usize> =
        std::collections::HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let games = std::fs::read_to_string(games_path).expect("Expect games filepath for parsing");
    for game in games.lines() {
        let (id, scores) = split_game_info(game);
        if valid_game(&scores, &constraints) {
            cum_sum += id;
        }
    }
    cum_sum
}

fn main() {
    println!(
        "{}",
        sum_of_possible_games("aoc_2023/day_two_cube/data/input1.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        assert_eq!(sum_of_possible_games("data/example1.txt"), 8);
    }
}
