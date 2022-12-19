use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");

struct Rucksack {
    first: String,
    second: String,
}

impl Rucksack {
    fn new(first: &str, second: &str) -> Self {
        Self {
            first: first.to_string(),
            second: second.to_string(),
        }
    }
}

fn calculate_priority(item: &char) -> Option<u32> {
    match item {
        ('a'..='z') => {
            for (i, c) in ('a'..='z').enumerate() {
                if item == &c {
                    return Some(i as u32 + 1);
                }
            }
        }
        ('A'..='Z') => {
            for (i, c) in ('A'..='Z').enumerate() {
                if item == &c {
                    return Some(i as u32 + 27);
                }
            }
        }
        _ => panic!("Unrecognized character"),
    }
    None
}

fn read_input(input: &str) -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    for v in input.split('\n') {
        let (first, second) = v.split_at(v.len() / 2);
        rucksacks.push(Rucksack::new(first, second));
    }
    rucksacks
}

pub fn run() {
    println!("=== Day 3 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn part1(input: &str) -> u32 {
    let rucksacks = read_input(input);
    let mut total_priority = 0u32;
    for rucksack in rucksacks.iter() {
        let first_set = HashSet::<char>::from_iter(rucksack.first.chars());
        let second_set = HashSet::<char>::from_iter(rucksack.second.chars());
        let common_values: HashSet<&char> = first_set.intersection(&second_set).collect();
        total_priority += common_values
            .iter()
            .map(|&c| calculate_priority(c).unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();
    }
    println!("Day 3 part 1, solution: {}", &total_priority);
    total_priority
}

fn part2(input: &str) -> u32 {
    let rucksacks = read_input(input);
    let mut total_priority = 0u32;
    for rucksack_group in rucksacks.chunks(3) {
        if rucksack_group.len() != 3 {
            continue;
        }

        let first_set = HashSet::<char>::from_iter(
            format!("{}{}", rucksack_group[0].first, rucksack_group[0].second).chars(),
        );
        let second_set = HashSet::<char>::from_iter(
            format!("{}{}", rucksack_group[1].first, rucksack_group[1].second).chars(),
        );
        let third_set = HashSet::<char>::from_iter(
            format!("{}{}", rucksack_group[2].first, rucksack_group[2].second).chars(),
        );
        let common_values: HashSet<&char> = first_set.intersection(&second_set).collect();
        let common_values: HashSet<char> =
            common_values.iter().map(|&&c| c).collect::<HashSet<char>>();
        let common_values: HashSet<&char> = common_values.intersection(&third_set).collect();

        total_priority += common_values
            .iter()
            .map(|c| calculate_priority(c).unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>();
    }
    println!("Day 3 part 2, solution: {}", &total_priority);
    total_priority
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 7597);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 70);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 2607);
    }
}
