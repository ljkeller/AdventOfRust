use std::collections::{HashMap, HashSet};
use std::path::Path;

trait Symbol {
    fn is_symbol(&self) -> bool;
}

impl Symbol for char {
    fn is_symbol(&self) -> bool {
        // naive symbol check is suitable for prob 3
        !self.is_numeric() && *self != '.'
    }
}

// only called on valid r and c
fn adj_to_symbol(grid: &Vec<Vec<char>>, r: usize, c: usize, r_len: usize, c_len: usize) -> bool {
    if r > r_len || c > c_len {
        panic!("Invalid row {r} or col {c}: max row: {r_len}, max col: {c_len}");
    }

    // conventional 8-neighbor checking
    r > 0 && c > 0 && grid[r - 1][c - 1].is_symbol()
        || r > 0 && grid[r - 1][c].is_symbol()
        || r > 0 && c < c_len - 1 && grid[r - 1][c + 1].is_symbol()
        || c > 0 && grid[r][c - 1].is_symbol()
        || c < c_len - 1 && grid[r][c + 1].is_symbol()
        || r < r_len - 1 && c > 0 && grid[r + 1][c - 1].is_symbol()
        || r < r_len - 1 && grid[r + 1][c].is_symbol()
        || r < r_len - 1 && c < c_len - 1 && grid[r + 1][c + 1].is_symbol()
}

// if we simply used signed types for coordinates, the solution would be much cleaner!
fn star_neighbors(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    r_len: usize,
    c_len: usize,
) -> Vec<(usize, usize)> {
    if r > r_len || c > c_len {
        panic!("Invalid row {r} or col {c}: max row: {r_len}, max col: {c_len}");
    }

    let mut star_neighbors: Vec<(usize, usize)> = Vec::new();
    if r > 0 && c > 0 && grid[r - 1][c - 1] == '*' {
        star_neighbors.push((r - 1, c - 1));
    }
    if r > 0 && grid[r - 1][c] == '*' {
        star_neighbors.push((r - 1, c));
    }
    if r > 0 && c < c_len - 1 && grid[r - 1][c + 1] == '*' {
        star_neighbors.push((r - 1, c + 1));
    }
    if c > 0 && grid[r][c - 1] == '*' {
        star_neighbors.push((r, c - 1));
    }
    if c < c_len - 1 && grid[r][c + 1] == '*' {
        star_neighbors.push((r, c + 1));
    }
    if r < r_len - 1 && c > 0 && grid[r + 1][c - 1] == '*' {
        star_neighbors.push((r + 1, c - 1));
    }
    if r < r_len - 1 && grid[r + 1][c] == '*' {
        star_neighbors.push((r + 1, c));
    }
    if r < r_len - 1 && c < c_len - 1 && grid[r + 1][c + 1] == '*' {
        star_neighbors.push((r + 1, c + 1));
    }

    star_neighbors
}

fn gear_ratios<P: AsRef<Path>>(schematic: P) -> (usize, usize) {
    let mut gear_ratio_sum_p1 = 0;

    let schematic_grid: Vec<Vec<char>> = std::fs::read_to_string(schematic)
        .expect("Expect schematic file")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let (row_max, col_max) = (schematic_grid.len(), schematic_grid[0].len());
    // part 1 vars
    let mut digit_power: u32 = 0;
    let mut current_number: usize = 0;
    let mut current_number_adjacent_to_symbol = false;
    // part 2 vars
    let mut star_gear_candidates: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut star_gear_window: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..row_max {
        // Go in reverse to accumulate numbers in a simpler manner
        for c in (0..col_max).rev() {
            match schematic_grid[r][c] {
                '0'..='9' => {
                    current_number += (schematic_grid[r][c].to_digit(10).unwrap() as usize)
                        * usize::pow(10, digit_power);
                    digit_power += 1;

                    current_number_adjacent_to_symbol |=
                        adj_to_symbol(&schematic_grid, r, c, row_max, col_max);
                    if current_number_adjacent_to_symbol {
                        star_gear_window.extend(star_neighbors(
                            &schematic_grid,
                            r,
                            c,
                            row_max,
                            col_max,
                        ));
                    }
                }
                _ => {
                    if current_number_adjacent_to_symbol {
                        // this algo will only sum on digit->non-digit transitions
                        gear_ratio_sum_p1 += current_number;
                    }
                    if !star_gear_window.is_empty() {
                        for gear in star_gear_window.iter() {
                            if !star_gear_candidates.contains_key(gear) {
                                star_gear_candidates.insert(*gear, Vec::new());
                            }
                            star_gear_candidates
                                .get_mut(gear)
                                .expect("Expect vec for gear")
                                .push(current_number);
                        }
                    }

                    digit_power = 0;
                    current_number = 0;
                    current_number_adjacent_to_symbol = false;
                    star_gear_window.clear();
                }
            }
        }
    }

    (
        gear_ratio_sum_p1,
        star_gear_candidates
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v.iter().product::<usize>())
            .sum(),
    )
}

fn main() {
    println!(
        "(p1, p2) {:?}",
        gear_ratios("aoc_2023/day_three_gear_ratios/data/sample1.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_ex1() {
        assert_eq!(gear_ratios("data/ex1.txt").0, 4361);
    }

    #[test]
    fn check_ex2() {
        assert_eq!(gear_ratios("data/ex1.txt").1, 467835);
    }
}
