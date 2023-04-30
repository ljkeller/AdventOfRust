use day_one;
use day_two;
use day_three;
use day_four;
use day_five;
use day_six;
use day_seven;
use day_eight;

fn print_results(day: i8, part: i8, result: &str)
{
    if part == 1 { println!("--------------------------------"); }
    println!("Day {} part {} results: {}", day, part, result);
    if part == 2 { println!("--------------------------------"); }
}
fn main() {
    println!("Advent of Rust 2022!");

    print_results(1, 1, &day_one::calories_one("data/day_one.txt").to_string());
    print_results(1, 2, &day_one::calories_two("data/day_one.txt").to_string());

    print_results(2, 1, &day_two::strategy_one("data/day_two.txt").to_string());
    print_results(2, 2, &day_two::strategy_two("data/day_two.txt").to_string());

    print_results(3, 1, &day_three::rucksack_one("data/day_three.txt").to_string());
    print_results(3, 2, &day_three::rucksack_two("data/day_three.txt").to_string());

    print_results(4, 1, &day_four::interval_one("data/day_four.txt").to_string());
    print_results(4, 2, &day_four::interval_two("data/day_four.txt").to_string());

    print_results(5, 1, &day_five::supplies_one("data/day_five.txt"));
    print_results(5, 2, &day_five::supplies_two("data/day_five.txt"));

    print_results(6, 1, &day_six::subroutine_one("data/day_six.txt").to_string());
    print_results(6, 2, &day_six::subroutine_two("data/day_six.txt").to_string());

    print_results(7, 1, &day_seven::size_one("data/day_seven.txt").to_string());
    print_results(7, 2, &day_seven::tmp_two("data/day_seven.txt").to_string());

    print_results(8, 1, &day_eight::visible_one("data/day_eight.txt").to_string());
    print_results(8, 2, &day_eight::visible_two("data/day_eight.txt").to_string());

    println!("End of results");
}
