use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("./input.txt");

const TEST_INPUT: &str = include_str!("./test_input.txt");

const PART1_ROUNDS: u8 = 20;
const PART2_ROUNDS: u32 = 10000;

#[derive(Debug, Eq, PartialEq)]
enum OperationType {
    Plus,
    Multiple,
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    number: u8,
    items: Vec<u64>,
    operation_value: u64,
    operation_type: OperationType,
    division: u64,
    throw: HashMap<bool, u8>,
    inspects: u32,
    common_multiple: u32,
}

impl Monkey {
    fn new() -> Self {
        Self {
            number: 0,
            items: vec![],
            operation_value: 0,
            operation_type: OperationType::Plus,
            division: 0,
            throw: HashMap::<_, _>::new(),
            inspects: 0,
            common_multiple: 0,
        }
    }

    fn worry_level_with_division(&self, item: u64) -> u64 {
        let factor = if self.operation_value == 0 {
            item
        } else {
            self.operation_value
        };
        match self.operation_type {
            OperationType::Plus => {
                let worry_level = (item + factor) as f64 / 3.0;
                worry_level.floor() as u64
            }
            OperationType::Multiple => {
                let worry_level = (item * factor) as f64 / 3.0;
                worry_level.floor() as u64
            }
        }
    }

    fn worry_level(&self, item: u64, common_multiple: u64) -> u64 {
        let factor = if self.operation_value == 0 {
            item
        } else {
            self.operation_value
        };
        match self.operation_type {
            OperationType::Plus => (item + factor) % common_multiple,
            OperationType::Multiple => (item * factor) % common_multiple,
        }
    }

    fn test(&self, value: u64) -> u8 {
        if value % self.division == 0 {
            self.throw[&true]
        } else {
            self.throw[&false]
        }
    }

    fn clear(&mut self) {
        self.items.clear();
    }

    fn add_inspects(&mut self) {
        self.inspects += self.items.len() as u32;
    }
}

fn read_monkeys(input: &str) -> HashMap<u8, RefCell<Monkey>> {
    let mut monkeys = HashMap::<u8, RefCell<Monkey>>::new();
    let re_monkey_num = Regex::new(r"^Monkey (\d):$").unwrap();
    let re_monkey_items = Regex::new(r"\s*Starting items: (.*)$").unwrap();
    let re_monkey_operation = Regex::new(r"\s*Operation: new = old ([+*]) (\d{1,2}|old)$").unwrap();
    let re_monkey_division = Regex::new(r"\s*Test: divisible by (\d{1,2})$").unwrap();
    let re_monkey_test = Regex::new(r"\s*If (true|false): throw to monkey (\d{1,2})$").unwrap();
    for c in input.split("\n\n") {
        let mut monkey = Monkey::new();
        for data in c.lines() {
            let data = data.trim();
            if re_monkey_num.is_match(data) {
                for cap in re_monkey_num.captures_iter(data) {
                    monkey.number = cap[1].parse::<u8>().unwrap();
                }
                continue;
            }
            if re_monkey_items.is_match(data) {
                for cap in re_monkey_items.captures_iter(data) {
                    for num in cap[1].split(',') {
                        monkey.items.push(num.trim().parse::<u64>().unwrap());
                    }
                }
                continue;
            }
            if re_monkey_operation.is_match(data) {
                for cap in re_monkey_operation.captures_iter(data) {
                    match &cap[1] {
                        "+" => monkey.operation_type = OperationType::Plus,
                        "*" => monkey.operation_type = OperationType::Multiple,
                        _ => {}
                    }
                    if &cap[2] == "old" {
                        monkey.operation_value = 0
                    } else {
                        monkey.operation_value = cap[2].parse::<u64>().unwrap();
                    }
                }
                continue;
            }
            if re_monkey_division.is_match(data) {
                for cap in re_monkey_division.captures_iter(data) {
                    monkey.division = cap[1].parse::<u64>().unwrap();
                }
                continue;
            }
            if re_monkey_test.is_match(data) {
                for cap in re_monkey_test.captures_iter(data) {
                    if &cap[1] == "true" {
                        monkey.throw.insert(true, cap[2].parse::<u8>().unwrap());
                    }
                    if &cap[1] == "false" {
                        monkey.throw.insert(false, cap[2].parse::<u8>().unwrap());
                    }
                }
                continue;
            }
        }
        monkeys.insert(monkey.number, RefCell::new(monkey));
    }
    monkeys
}

pub fn run() {
    println!("=== Day 11 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn calculate_monkey_business(monkeys: &HashMap<u8, RefCell<Monkey>>) -> u64 {
    monkeys
        .iter()
        .map(|(_, monkey)| monkey.borrow().inspects as u64)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn part1(input: &str) -> u64 {
    let monkeys = read_monkeys(input);
    for _ in 0..PART1_ROUNDS {
        for (_, monkey) in monkeys.iter().sorted_by_key(|x| x.0) {
            if monkey.borrow().items.is_empty() {
                continue;
            }
            monkey.borrow_mut().add_inspects();
            for item in &monkey.borrow().items {
                let worry_level = monkey.borrow().worry_level_with_division(*item);
                let throw_to = monkey.borrow().test(worry_level);
                monkeys
                    .get(&throw_to)
                    .unwrap()
                    .borrow_mut()
                    .items
                    .push(worry_level);
            }
            monkey.borrow_mut().clear();
        }
    }
    let result = calculate_monkey_business(&monkeys);
    println!("Day 11 part 1, solution: {}", result);
    result
}

fn part2(input: &str) -> u64 {
    let monkeys = read_monkeys(input);
    let common_multiple: u64 = monkeys.iter().map(|m| m.1.borrow().division).product();
    dbg!(common_multiple);
    for _ in 0..PART2_ROUNDS {
        for (_, monkey) in monkeys.iter().sorted_by_key(|x| x.0) {
            if monkey.borrow().items.is_empty() {
                continue;
            }
            monkey.borrow_mut().add_inspects();
            for item in &monkey.borrow().items {
                let worry_level: u64 = monkey.borrow().worry_level(*item, common_multiple);
                let throw_to = monkey.borrow().test(worry_level);
                monkeys
                    .get(&throw_to)
                    .unwrap()
                    .borrow_mut()
                    .items
                    .push(worry_level);
            }
            monkey.borrow_mut().clear();
        }
    }
    let result = calculate_monkey_business(&monkeys);
    println!("Day 11 part 2, solution: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 55944);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 15117269860);
    }
}
