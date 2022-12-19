use std::collections::HashSet;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("./input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");

struct Pair {
    start: u32,
    end: u32,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pair({}, {})", self.start, self.end)
    }
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once("-")
            .expect(format!("Could not split str {} with '-'", s).as_str());

        let start_asu32 = start.parse::<u32>()?;
        let end_asu32 = end.parse::<u32>()?;

        Ok(Self {
            start: start_asu32,
            end: end_asu32,
        })
    }
}

struct AssignmentPair {
    first: Pair,
    second: Pair,
}

impl AssignmentPair {
    fn new(first: Pair, second: Pair) -> Self {
        Self { first, second }
    }
}

impl fmt::Display for AssignmentPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AssignmentPair({}, {})", self.first, self.second)
    }
}

fn read_input(input: &str) -> Vec<AssignmentPair> {
    let mut pairs: Vec<AssignmentPair> = Vec::new();
    for v in input.split('\n') {
        let split = v
            .split_once(",")
            .expect(format!("Could not split line {} with ','", v).as_str());
        let p1 = Pair::from_str(split.0)
            .expect(format!("Could not create Pair from str: {}", split.0).as_str());
        let p2 = Pair::from_str(split.1)
            .expect(format!("Could not create Pair from str: {}", split.1).as_str());

        pairs.push(AssignmentPair::new(p1, p2));
    }
    pairs
}

pub fn run() {
    println!("=== Day 4 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn part1(input: &str) -> u32 {
    let pairs = read_input(input);
    let mut fully_contains = 0u32;
    for a_pair in pairs.iter() {
        let first_range = (a_pair.first.start..=a_pair.first.end).collect::<Vec<u32>>();
        let second_range = (a_pair.second.start..=a_pair.second.end).collect::<Vec<u32>>();
        if first_range.first().unwrap() <= second_range.first().unwrap()
            && first_range.last().unwrap() >= second_range.last().unwrap()
            || first_range.first().unwrap() >= second_range.first().unwrap()
                && first_range.last().unwrap() <= second_range.last().unwrap()
        {
            fully_contains += 1;
        }
    }
    println!("Day 4 part 1, solution: {}", &fully_contains);
    fully_contains
}

fn part2(input: &str) -> u32 {
    let pairs = read_input(input);
    let mut fully_contains = 0u32;
    for a_pair in pairs.iter() {
        let first_set = (a_pair.first.start..=a_pair.first.end).collect::<HashSet<u32>>();
        let second_set = (a_pair.second.start..=a_pair.second.end).collect::<HashSet<u32>>();
        if first_set.intersection(&second_set).next().is_some() {
            fully_contains += 1
        }
    }
    println!("Day 4 part 2, solution: {}", &fully_contains);
    fully_contains
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 471);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 4);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 888);
    }
}
