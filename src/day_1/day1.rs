use itertools::Itertools;
use std::cmp::Reverse;

const INPUT: &str = include_str!("./input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");

pub fn run() {
    println!("=== Day 1 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn part1(input: &str) -> u64 {
    let max = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .max();
    println!("Day 1 part 1, solution: {}", max.unwrap_or(0));
    max.unwrap_or(0)
}

fn part2(input: &str) -> u64 {
    let sum = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|v| v.0)
        .sum();
    println!("Day 1 part 2, solution: {}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 24000);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 68787);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 45000);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 198041);
    }
}
