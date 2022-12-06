use std::collections::VecDeque;
use std::iter;

use regex::Regex;

const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test_input.txt");
const TEST_STACK: &str = "N,Z;D,C,M;P";
const STACK: &str = "R,H,M,P,Z;B,J,C,P;D,C,L,G,H,N,S;L,R,S,Q,D,M,T,F;M,Z,T,B,Q,P,S,F;G,B,Z,S,F,T;V,R,N;M,C,V,D,T,L,G,P;L,M,F,J,N,Q,W";

fn create_crates_for_test_input(stack_sequence: &'static str) -> Vec<VecDeque<&'static str>> {
    let mut crates = Vec::<VecDeque<&'static str>>::new();
    for c in stack_sequence.split(';') {
        let mut deque = VecDeque::<&str>::new();
        for s in c.split(',') {
            deque.push_front(s);
        }
        crates.push(deque)
    }
    crates
}

pub fn run() {
    println!("=== Day 5 ===");
    let mut crates = create_crates_for_test_input(STACK);
    part1(INPUT, &mut crates.clone());
    part2(INPUT, &mut crates);
    println!("=============");
    println!();
}

fn part1(input: &str, crates: &mut Vec<VecDeque<&str>>) -> String {
    let re = Regex::new(r"^move (\d{1,2}) from (\d{1,2}) to (\d{1,2})$")
        .expect("Unable to create regex");
    for line in input.split('\n') {
        if line.contains("move") {
            for cap in re.captures_iter(line.trim()) {
                let repeat = &cap[1].parse::<u32>().unwrap();
                let from = &cap[2].parse::<u32>().unwrap();
                let to = &cap[3].parse::<u32>().unwrap();
                for _ in iter::repeat(repeat).take(*repeat as usize) {
                    if let Some(deque) = crates.get_mut((*from - 1) as usize) {
                        let item = deque.pop_back().unwrap();
                        if let Some(_deque) = crates.get_mut((*to - 1) as usize) {
                            _deque.push_back(item)
                        }
                    }
                }
            }
        }
    }
    let mut message = String::from("");
    for c in crates {
        message.push_str(c.pop_back().unwrap())
    }
    println!("Day 5 part 1, solution: {}", &message);
    message
}

fn part2(input: &str, crates: &mut Vec<VecDeque<&str>>) -> String {
    let re = Regex::new(r"^move (\d{1,2}) from (\d{1,2}) to (\d{1,2})$")
        .expect("Unable to create regex");
    for line in input.split('\n') {
        if line.contains("move") {
            for cap in re.captures_iter(line.trim()) {
                let repeat = &cap[1].parse::<u32>().unwrap();
                let mut tmp_deque = VecDeque::<&str>::new();
                let from = &cap[2].parse::<u32>().unwrap();
                let to = &cap[3].parse::<u32>().unwrap();
                for _ in iter::repeat(repeat).take(*repeat as usize) {
                    if let Some(deque) = crates.get_mut((*from - 1) as usize) {
                        tmp_deque.push_front(deque.pop_back().unwrap());
                    }
                }
                if let Some(_deque) = crates.get_mut((*to - 1) as usize) {
                    for &c in tmp_deque.iter() {
                        _deque.push_back(c)
                    }
                }
            }
        }
    }
    let mut message = String::from("");
    for c in crates {
        message.push_str(c.pop_back().unwrap())
    }
    println!("Day 5 part 2, solution: {}", &message);
    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        let mut crates = create_crates_for_test_input(TEST_STACK);
        assert_eq!(part1(TEST_INPUT, &mut crates), "CMZ");
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        let mut crates = create_crates_for_test_input(STACK);
        assert_eq!(part1(INPUT, &mut crates), "VQZNJMWTR");
    }

    #[test]
    fn part2_works_with_test_input() {
        let mut crates = create_crates_for_test_input(TEST_STACK);
        assert_eq!(part2(TEST_INPUT, &mut crates), "MCD");
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        let mut crates = create_crates_for_test_input(STACK);
        assert_eq!(part2(INPUT, &mut crates), "NLCDCLVMQ");
    }
}
