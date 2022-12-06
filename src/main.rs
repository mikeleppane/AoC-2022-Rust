mod day_3;
mod day_4;
mod day_5;
mod day_6;

use day_3::day3;
use day_4::day4;
use day_5::day5;
use day_6::day6;

fn main() {
    println!("======*****======= AoC 2022 Solutions ======*****=======");
    println!();
    println!();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
}
