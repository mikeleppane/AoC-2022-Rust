#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
const INPUT: &str = include_str!("./input.txt");

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Sequence {
    pub(crate) numbers: Vec<isize>,
}

impl Sequence {
    fn new(numbers: Vec<isize>) -> Self {
        Self { numbers }
    }
    fn add(&mut self, num: isize) {
        let pos = self
            .numbers
            .iter()
            .position(|&v| v == num)
            .unwrap_or_else(|| panic!("Cannot find number from sequence: {}", num));

        let mut index = pos;
        let max_index = self.numbers.len() - 1;
        if num < 0 {
            for _ in 0..num.abs() {
                if index == 0 {
                    index = max_index;
                    continue;
                }
                index -= 1;
                if index == 0 {
                    index = max_index;
                }
            }
            self.numbers.remove(pos);
            self.numbers.insert(index, num);
        } else {
            for _ in 0..num {
                if index == max_index {
                    index = 0;
                    continue;
                }
                index += 1;
                if index == max_index {
                    index = 0;
                }
            }
            self.numbers.remove(pos);
            self.numbers.insert(index, num);
        }
    }
}

impl std::fmt::Debug for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");
        for num in &self.numbers {
            output.push_str(format!("{}\n", num).as_str());
        }
        write!(f, "{}", output.as_str())
    }
}

fn read_file(input: &str) -> Sequence {
    let mut numbers = Vec::<isize>::new();
    for c in input.lines() {
        numbers.push(
            c.trim()
                .parse::<isize>()
                .expect(format!("Could not parse the given number {}", c).as_str()),
        )
    }
    Sequence::new(numbers)
}

pub fn run() {
    println!("=== Day 20 ===");
    println!("Day 20 part 1, solution: {}", part1(INPUT));
    // println!(
    //     "Day 20 part 2, solution: {}",
    //     solve(INPUT, 1_000_000_000_000)
    // );
    println!("=============");
    println!();
}

fn part1(input: &str) -> isize {
    let mut sequence = read_file(input);
    let encrypted_file = sequence.numbers.clone();
    for num in &encrypted_file {
        if *num == 0 {
            continue;
        }
        sequence.add(*num);
    }
    let zero_pos = sequence
        .numbers
        .iter()
        .position(|&v| v == 0)
        .expect("Cannot find 0 from the sequence");

    let mut index = zero_pos;
    let mut coordinates = 0isize;
    let numbers = [1000, 2000, 3000];
    for n in 1..=3000 {
        if index == sequence.numbers.len() - 1 {
            index = 0;
            continue;
        }
        index += 1;
        if numbers.contains(&n) {
            coordinates += sequence.numbers[index];
        }
    }
    coordinates
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 3);
    }

    // #[test]
    // fn part1_works_with_puzzle_input() {
    //     assert_eq!(solve(INPUT, 2022), 3098);
    // }
}
