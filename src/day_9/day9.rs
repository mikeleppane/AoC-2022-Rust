use std::cell::RefCell;
use std::collections::HashSet;
use std::iter;
use std::num::ParseIntError;
use std::str::FromStr;

const TEST_INPUT: &str = include_str!("./test_input.txt");
const TEST_INPUT_2: &str = include_str!("./test_input1_p2.txt");
const INPUT: &str = include_str!("./input.txt");

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Coord {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Head {
    pub(crate) coord: Coord,
}

impl Head {
    fn new() -> Self {
        Self {
            coord: Coord::from((0, 0)),
        }
    }

    fn move_head(&mut self, command: Command, tails: &[RefCell<Tail>]) {
        for step in iter::repeat(1).take(command.steps) {
            match command.direction {
                Direction::Up => self.coord.y += step as isize,
                Direction::Down => self.coord.y -= step as isize,
                Direction::Right => self.coord.x += step as isize,
                Direction::Left => self.coord.x -= step as isize,
            }
            let mut new_coord = self.coord.clone();
            for (index, tail) in tails.iter().enumerate() {
                if index == 0 {
                    tail.borrow_mut().move_tail(&new_coord);
                    continue;
                }
                let nn = tails[index - 1].borrow().get_pos();
                new_coord = tail.borrow_mut().move_tail(&nn);
            }
        }
    }
}

impl std::fmt::Debug for Head {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.coord.x, self.coord.y)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Tail {
    pub(crate) coord: Coord,
    pub(crate) visited_positions: HashSet<Coord>,
}

impl std::fmt::Debug for Tail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinates({}, {})", self.coord.x, self.coord.y)
    }
}

impl Tail {
    fn new() -> Self {
        let mut visited_positions = HashSet::new();
        visited_positions.insert(Coord::from((0, 0)));
        Self {
            coord: Coord::from((0, 0)),
            visited_positions,
        }
    }

    fn rel_move(&mut self, new_pos: (isize, isize)) {
        self.coord.x += new_pos.0;
        self.coord.y += new_pos.1;
    }

    fn get_pos(&self) -> Coord {
        self.coord.clone()
    }

    fn move_tail(&mut self, head_coord: &Coord) -> Coord {
        if self.coord.x == head_coord.x
            && (self.coord.y < head_coord.y && self.coord.y.abs_diff(head_coord.y) > 1)
        {
            self.coord.y += 1;
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        if self.coord.x == head_coord.x
            && (self.coord.y > head_coord.y && self.coord.y.abs_diff(head_coord.y) > 1)
        {
            self.coord.y -= 1;
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        if self.coord.y == head_coord.y
            && (self.coord.x < head_coord.x && self.coord.x.abs_diff(head_coord.x) > 1)
        {
            self.coord.x += 1;
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        if self.coord.y == head_coord.y
            && (self.coord.x > head_coord.x && self.coord.x.abs_diff(head_coord.x) > 1)
        {
            self.coord.x -= 1;
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        if head_coord.y - self.coord.y > 1 {
            if self.coord.x < head_coord.x {
                self.rel_move((1, 1))
            } else {
                self.rel_move((-1, 1))
            }
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        if self.coord.y - head_coord.y > 1 {
            if self.coord.x < head_coord.x {
                self.rel_move((1, -1))
            } else {
                self.rel_move((-1, -1))
            }
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        if head_coord.x - self.coord.x > 1 {
            if self.coord.y < head_coord.y {
                self.rel_move((1, 1))
            } else {
                self.rel_move((1, -1))
            }
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        if self.coord.x - head_coord.x > 1 {
            if self.coord.y < head_coord.y {
                self.rel_move((-1, 1))
            } else {
                self.rel_move((-1, -1))
            }
            self.visited_positions.insert(self.coord.clone());
            return self.coord.clone();
        }
        self.coord.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "r" => Ok(Self::Right),
            "l" => Ok(Self::Left),
            "u" => Ok(Self::Up),
            "d" => Ok(Self::Down),
            _ => panic!("Unrecognized direction {} encountered!", s),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Command {
    pub(crate) direction: Direction,
    pub(crate) steps: usize,
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s
            .split_once(' ')
            .unwrap_or_else(|| panic!("Could not split str {} ", s));
        let steps = steps.parse::<u32>()?;

        Ok(Self {
            direction: Direction::from_str(direction)
                .unwrap_or_else(|_| panic!("Could not create direction from {} ", direction)),
            steps: steps as usize,
        })
    }
}

fn read_input(input: &str) -> Vec<Command> {
    let mut commands = Vec::<Command>::new();
    for line in input.lines() {
        let command = Command::from_str(line.trim());
        commands.push(
            command
                .unwrap_or_else(|_| panic!("Failure when creating command from a line: {}", line)),
        )
    }
    commands
}

pub fn run() {
    println!("=== Day 9 ===");
    println!("Day 9 part 1, solution: {}", part1(INPUT));
    println!("Day 9 part 2, solution: {}", part2(INPUT));
    println!("=============");
    println!();
}

fn part1(input: &str) -> usize {
    let commands = read_input(input);
    let mut head = Head::new();
    let tails = vec![RefCell::new(Tail::new())];
    for command in commands {
        head.move_head(command, &tails)
    }

    let visited_places_by_tail = tails.last().unwrap().borrow().visited_positions.len();
    visited_places_by_tail
}

fn part2(input: &str) -> usize {
    let commands = read_input(input);
    let mut head = Head::new();
    let mut tails = Vec::<RefCell<Tail>>::new();
    for _ in 0..9 {
        tails.push(RefCell::new(Tail::new()));
    }
    for command in commands {
        head.move_head(command, &tails)
    }
    let visited_places_by_tail = tails.last().unwrap().borrow().visited_positions.len();
    visited_places_by_tail
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 6175);
    }
    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 1);
    }

    #[test]
    fn part2_works_with_test_input_2() {
        assert_eq!(part2(TEST_INPUT_2), 36);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 2578);
    }
}
