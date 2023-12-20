use std::{path::Path, thread::current};

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

fn gear_ratios<P: AsRef<Path>>(schematic: P) -> usize {
    let mut gear_ratio_sum = 0;

    let schematic_grid: Vec<Vec<char>> = std::fs::read_to_string(schematic)
        .expect("Expect schematic file")
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let (row_max, col_max) = (schematic_grid.len(), schematic_grid[0].len());
    let mut digit_power: u32 = 0;
    let mut current_number: usize = 0;
    let mut current_number_adjacent_to_symbol = false;
    for r in 0..row_max {
        // Go in reverse to accumulate numbers in a simpler manner
        for c in (0..col_max).rev() {
            match schematic_grid[r][c] {
                '0'..='9' => {
                    current_number += (schematic_grid[r][c].to_digit(10).unwrap() as usize)
                        * usize::pow(10, digit_power);
                    digit_power += 1;

                    // check neighbors
                    current_number_adjacent_to_symbol |=
                        adj_to_symbol(&schematic_grid, r, c, row_max, col_max);
                }
                _ => {
                    if current_number_adjacent_to_symbol {
                        // this algo will only sum on digit->non-digit transitions
                        gear_ratio_sum += current_number;
                    }

                    digit_power = 0;
                    current_number = 0;
                    current_number_adjacent_to_symbol = false
                }
            }
        }
    }
    gear_ratio_sum
}

fn main() {
    println!(
        "p1 {}",
        gear_ratios("aoc_2023/day_three_gear_ratios/data/sample1.txt")
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_ex1() {
        assert_eq!(gear_ratios("data/ex1.txt"), 4361);
    }
}
