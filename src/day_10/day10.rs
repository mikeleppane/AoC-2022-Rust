#[allow(dead_code)]

const INPUT: &str = include_str!("./input.txt");
#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");

#[derive(Debug)]
enum Instruction {
    Addx,
    Noop,
}

#[derive(Debug)]
struct Program {
    inst_type: Instruction,
    value: Option<i32>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Crt {
    data: Vec<Vec<&'static str>>,
    size: (u8, u8),
}

impl Crt {
    #[allow(dead_code)]
    fn new(new_value: &'static str, size: (u8, u8)) -> Self {
        let mut crt = Vec::<Vec<&'static str>>::new();
        let row = vec![new_value; (size.0 * size.1) as usize];
        for _ in 0..size.0 {
            crt.push(row.clone());
        }
        Self { data: crt, size }
    }

    fn index(&self, n: usize) -> &'static str {
        let row = n / self.size.1 as usize;
        let column = n % self.size.1 as usize;
        self.data[row][column]
    }

    fn indices(&self, n: usize) -> (u8, u8) {
        let row = (n / self.size.1 as usize) as u8;
        let column = (n % self.size.1 as usize) as u8;
        (row, column)
    }
}

impl Program {
    fn new(inst_type: Instruction, value: Option<i32>) -> Self {
        Self { inst_type, value }
    }
}

fn read_input(input: &str) -> Vec<Program> {
    let mut program = Vec::<Program>::new();
    for line in input.split("\n") {
        let line: Vec<&str> = line.trim().split(" ").collect();
        match line[0] {
            "addx" => program.push(Program::new(
                Instruction::Addx,
                Some(
                    line[1]
                        .parse::<i32>()
                        .unwrap_or_else(|_| panic!("Could not parse value ({}) to i32", line[1])),
                ),
            )),
            "noop" => program.push(Program::new(Instruction::Noop, None)),
            _ => {
                panic!("Unrecognized instruction ({}) encountered!", line[0])
            }
        }
    }
    program
}

pub fn run() {
    println!("=== Day 10 ===");
    println!("Day 10 part 1, solution: {}", part1(INPUT));
    //println!("Day 10 part 2, solution: {}", solve(INPUT, 14));
    println!("=============");
    println!();
}

fn calculate_signal_strength(cycle: i32, register: i32) -> i32 {
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => {
            dbg!(register * cycle);
            register * cycle
        }
        _ => 0,
    }
}

fn part1(input: &str) -> i32 {
    let program = read_input(input);
    let mut cycles = 0;
    let mut register = 1;
    let mut signal_strength = 0;
    for instruction in program {
        match instruction.inst_type {
            Instruction::Addx => {
                for cycle in [1, 1] {
                    cycles += cycle;
                    signal_strength += calculate_signal_strength(cycles, register);
                }
                register += instruction.value.unwrap();
            }
            Instruction::Noop => {
                cycles += 1;
                signal_strength += calculate_signal_strength(cycles, register);
            }
        }
    }
    signal_strength
}

fn part2(_input: &str) {
    let _crt = Crt::new("#", (6, 40));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 14540);
    }

    #[test]
    fn part2_works_with_test_input() {
        part2(TEST_INPUT)
    }
}
