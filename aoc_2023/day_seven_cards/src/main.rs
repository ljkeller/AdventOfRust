use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: String,
    type_rank: u8,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_strength = HashMap::<char, usize>::from([
            ('2', 1),
            ('3', 2),
            ('4', 3),
            ('5', 4),
            ('6', 5),
            ('7', 6),
            ('8', 7),
            ('9', 8),
            ('T', 9),
            ('J', 10),
            ('Q', 11),
            ('K', 12),
            ('A', 13),
        ]);

        if self.type_rank != other.type_rank {
            return self.type_rank.cmp(&other.type_rank);
        } else {
            for (a, b) in self.cards.chars().zip(other.cards.chars()) {
                if a != b {
                    return card_strength
                        .get(&a)
                        .unwrap()
                        .cmp(card_strength.get(&b).unwrap());
                }
            }

            return std::cmp::Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_rank(hand: &str) -> u8 {
    let mut count = HashMap::<char, usize>::new();
    for c in hand.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    match count.len() {
        1 => return 7, // five of a kind
        4 => return 2, // one pair
        5 => return 1, // high card
        2 => {
            // four of kind or full house
            let mut sorted = count.values().into_iter().sorted().rev();
            let most_freq_card_count = *sorted.next().unwrap();
            match most_freq_card_count {
                4 => 6,
                3 => 5,
                _ => panic!("Invalid max freq for hand type with two distinct cards"),
            }
        }
        3 => {
            // three of a kind or two pair
            let mut sorted = count.values().into_iter().sorted().rev();
            let most_freq_card_count = *sorted.next().unwrap();
            match most_freq_card_count {
                3 => 4,
                2 => 3,
                _ => panic!("Invalid max freq for hand type with 3 distinct cards"),
            }
        }
        _ => {
            panic!("Bad card rank!")
        }
    }
}

fn parse_cards(cards: &str) -> Vec<Hand> {
    let mut hands = Vec::<Hand>::new();

    for card_row in cards.lines() {
        let (hand, bid) = card_row.split_once(" ").expect("Expect hand split at ' '");
        hands.push(Hand {
            cards: hand.to_string(),
            type_rank: get_rank(hand),
            bid: bid.parse::<usize>().expect("Expect bid parse"),
        });
    }

    hands
}

fn total_winnings<P: AsRef<Path>>(cards_p: P) -> (usize, usize) {
    let cards_data = std::fs::read_to_string(cards_p).expect("Expect card data");
    let mut hands = parse_cards(&cards_data);
    hands.sort();
    (
        hands
            .into_iter()
            .enumerate()
            .map(|(idx, hand)| (idx + 1) * hand.bid)
            .sum(),
        0,
    )
}

fn main() {
    println!(
        "(p1, p1) {:?}",
        total_winnings("aoc_2023/day_seven_cards/data/sample1.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(total_winnings("data/ex1.txt").0, 6440)
    }

    #[test]
    fn ex2() {
        assert_eq!(total_winnings("data/ex1.txt").1, 0)
    }
}
