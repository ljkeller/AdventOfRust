use std::{path::Path, str::FromStr};

trait Ints {
    fn ints<F: FromStr>(&self) -> Vec<F>;
}

impl Ints for str {
    fn ints<F: FromStr>(&self) -> Vec<F> {
        let mut ints = Vec::new();
        let toks = self.split_ascii_whitespace();
        toks.for_each(|t| {
            let p = t.parse::<F>();
            match p {
                Ok(p) => ints.push(p),
                _ => {}
            }
        });

        ints
    }
}

trait BigInt {
    fn bigint(&self) -> usize;
}

impl BigInt for str {
    fn bigint(&self) -> usize {
        let mut bigstr = String::new();
        let toks = self.split_ascii_whitespace();
        toks.for_each(|t| {
            let n = t.parse::<usize>();
            match n {
                Ok(n) => bigstr.push_str(n.to_string().as_str()),
                _ => {}
            }
        });

        bigstr.parse::<usize>().expect("Expect bigint reduction")
    }
}

fn parse_race_data(data: &str) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut toks = data.split(": ");
    let times = toks
        .nth(1)
        .expect("Expect times after 'time: '")
        .ints::<usize>();
    let dists = toks
        .next()
        .expect("Expect times after ': '")
        .ints::<usize>();

    let mut big_toks = data.split(": ");
    let big_time = big_toks.nth(1).unwrap().bigint();
    let big_dist = big_toks.next().unwrap().bigint();

    (
        times
            .iter()
            .zip(dists.iter())
            .map(|(&a, &b)| (a, b))
            .collect(),
        vec![(big_time, big_dist)],
    )
}

fn simulate_wins(races: Vec<(usize, usize)>) -> Vec<usize> {
    let mut win_data = Vec::new();

    for (time, win_dist) in races.into_iter() {
        let mut win_count = 0;

        for hold_secs in 1..time {
            let dist = hold_secs * (time - hold_secs);
            if dist > win_dist {
                win_count += 1;
            }
        }

        win_data.push(win_count);
    }

    win_data
}

fn solve_boat_race<P: AsRef<Path>>(boat_constraints_p: P) -> (usize, usize) {
    let race_data = std::fs::read_to_string(boat_constraints_p).expect("Expect boat fp");
    let (races, big_race) = parse_race_data(&race_data);
    let win_cdns = simulate_wins(races);
    let big_race_cdns = simulate_wins(big_race);

    (
        win_cdns.into_iter().product(),
        *big_race_cdns
            .last()
            .expect("Expect 1 win count for big race"),
    )
}

fn main() {
    println!(
        "(p1, p1) {:?}",
        solve_boat_race("aoc_2023/day_six_boat/data/sample1.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(solve_boat_race("data/ex1.txt").0, 288)
    }

    #[test]
    fn ex2() {
        assert_eq!(solve_boat_race("data/ex1.txt").1, 71503)
    }
}
