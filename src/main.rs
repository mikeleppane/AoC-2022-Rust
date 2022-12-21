extern crate core;

mod day_10;
mod day_14;
mod day_17;
mod day_20;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_8;
mod day_9;

use day_10::day10;
use day_14::day14;
use day_17::day17;
use day_20::day20;
use day_3::day3;
use day_4::day4;
use day_5::day5;
use day_6::day6;
use day_8::day8;
use day_9::day9;

fn main() {
    println!("======*****======= AoC 2022 Solutions ======*****=======");
    println!();
    println!();
    day3::run();
    day4::run();
    day5::run();
    day6::run();
    day8::run();
    day9::run();
    day10::run();
    day14::run();
    day17::run();
    day20::run();
}
