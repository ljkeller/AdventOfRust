use std::path::Path;

struct Almanac {}
trait Mappable {
    fn to_location(&mut self, global_map: Almanac);
}

impl Mappable for Vec<usize> {
    fn to_location(&mut self, global_map: Almanac) {}
}

fn parse_seed_data(data: &Vec<&str>) -> (Vec<usize>, Almanac) {
    let mut seeds = Vec::new();
    let mut almanac = Almanac {};

    (seeds, almanac)
}

fn solve_seed_mapping<P: AsRef<Path>>(seed_path: P) -> (usize, usize) {
    let seeds_data = std::fs::read_to_string(seed_path).expect("Expect seed path");
    let data_lines: Vec<&str> = seeds_data.lines().into_iter().collect();

    let (mut seeds, global_map) = parse_seed_data(&data_lines);

    seeds.to_location(global_map);

    (seeds.into_iter().min().expect("Expect min location"), 0)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(solve_seed_mapping("data/ex1.txt").0, 35);
    }

    // #[test]
    // fn ex2() {
    // }
}
