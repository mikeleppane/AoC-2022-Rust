use itertools::Itertools;
use std::cmp;
use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
const INPUT: &str = include_str!("./input.txt");

type Tower = HashSet<Coord>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum RockType {
    Horizontal,
    Cross,
    MirrorL,
    Vertical,
    Square,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coord {
    fn add(&mut self, coord: (isize, isize)) {
        self.x = self.x.checked_add_signed(coord.0).unwrap();
        self.y = self.y.checked_add_signed(coord.1).unwrap();
    }

    fn get(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Rock {
    pub(crate) positions: Vec<Coord>,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl Rock {
    fn new(rock_type: RockType, starting_pos: (usize, usize)) -> Self {
        match rock_type {
            RockType::Horizontal => {
                let mut positions = Vec::<Coord>::new();
                for pos in [(0, 0), (1, 0), (2, 0), (3, 0)] {
                    let mut new_place = Coord::from(starting_pos);
                    new_place.add(pos);
                    positions.push(new_place);
                }
                Self {
                    positions,
                    height: 1,
                    width: 4,
                }
            }
            RockType::Cross => {
                let mut positions = Vec::<Coord>::new();
                for pos in [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)] {
                    let mut new_place = Coord::from(starting_pos);
                    new_place.add(pos);
                    positions.push(new_place);
                }
                Self {
                    positions,
                    height: 3,
                    width: 3,
                }
            }
            RockType::MirrorL => {
                let mut positions = Vec::<Coord>::new();
                for pos in [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)] {
                    let mut new_place = Coord::from(starting_pos);
                    new_place.add(pos);
                    positions.push(new_place);
                }
                Self {
                    positions,
                    height: 3,
                    width: 3,
                }
            }
            RockType::Vertical => {
                let mut positions = Vec::<Coord>::new();
                for pos in [(0, 0), (0, 1), (0, 2), (0, 3)] {
                    let mut new_place = Coord::from(starting_pos);
                    new_place.add(pos);
                    positions.push(new_place);
                }
                Self {
                    positions,
                    height: 4,
                    width: 1,
                }
            }
            RockType::Square => {
                let mut positions = Vec::<Coord>::new();
                for pos in [(0, 0), (0, 1), (1, 1), (1, 0)] {
                    let mut new_place = Coord::from(starting_pos);
                    new_place.add(pos);
                    positions.push(new_place);
                }
                Self {
                    positions,
                    height: 2,
                    width: 2,
                }
            }
        }
    }

    fn is_at_floor(&self) -> bool {
        for pos in &self.positions {
            if pos.y == 0 {
                return true;
            }
        }
        false
    }

    fn check_down_move(&self, incr: (isize, isize), tower: &Tower) -> bool {
        for mut pos in self.positions.clone() {
            if pos.y == 1 && incr.1 == -1 {
                return false;
            }
            pos.add(incr);
            if tower.contains(&pos) {
                return false;
            }
        }
        true
    }

    fn check_horizontal_move(&self, incr: (isize, isize), tower: &Tower) -> bool {
        for mut pos in self.positions.clone() {
            if pos.x == 1 && incr.0 == -1 || pos.x == 7 && incr.0 == 1 {
                return false;
            }
            pos.add(incr);
            if tower.contains(&pos) {
                return false;
            }
        }
        true
    }
    fn add(&mut self, incr: (isize, isize)) {
        for pos in &mut self.positions {
            pos.add(incr)
        }
    }
}

impl std::fmt::Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("");
        for pos in &self.positions {
            output.push_str(format!("({}, {})\n", pos.x, pos.y).as_str());
        }
        write!(f, "{}", output.as_str())
    }
}

fn read_jet_pattern(input: &str) -> Vec<char> {
    let mut pattern = Vec::<char>::new();
    for c in input.trim().chars() {
        pattern.push(c);
    }
    pattern
}

pub fn run() {
    println!("=== Day 17 ===");
    println!("Day 17 part 1, solution: {}", solve(INPUT, 2022));
    println!(
        "Day 14 part 2, solution: {}",
        solve(INPUT, 1_000_000_000_000)
    );
    println!("=============");
    println!();
}

fn calculate_starting_position(max_rock: usize) -> (usize, usize) {
    if max_rock == 0 {
        return (3, 4);
    }
    (3, max_rock + 4)
}

fn simulate_rock_fall(
    rock: &mut Rock,
    tower: &Tower,
    jet_pattern: &Vec<char>,
    jet_counter: &mut usize,
) {
    loop {
        let index = *jet_counter % jet_pattern.len();
        match jet_pattern[index] {
            '<' => {
                if rock.check_horizontal_move((-1, 0), tower) {
                    rock.add((-1, 0));
                }
            }
            '>' => {
                if rock.check_horizontal_move((1, 0), tower) {
                    rock.add((1, 0));
                }
            }
            _ => panic!(
                "Unrecognized jet pattern encountered! => {}",
                jet_pattern[index]
            ),
        }
        *jet_counter += 1;
        if rock.check_down_move((0, -1), tower) {
            rock.add((0, -1));
        } else {
            return;
        }
        if rock.is_at_floor() {
            return;
        }
    }
}

fn solve(input: &str, num_of_rocks: usize) -> usize {
    let pattern = read_jet_pattern(input);
    let rock_types = [
        RockType::Horizontal,
        RockType::Cross,
        RockType::MirrorL,
        RockType::Vertical,
        RockType::Square,
    ];

    let mut rock_counter = 1usize;
    let mut jet_counter = 0usize;
    let mut max_rock = 0usize;
    let mut rock_pos_cache = VecDeque::<Coord>::new();
    for r_type in rock_types.iter().cycle() {
        let starting_pos = calculate_starting_position(max_rock);
        let mut rock = Rock::new(r_type.clone(), starting_pos);
        let tower = HashSet::<_>::from_iter(rock_pos_cache.iter().cloned());
        simulate_rock_fall(&mut rock, &tower, &pattern, &mut jet_counter);
        for pos in rock.positions {
            max_rock = cmp::max(max_rock, pos.y);
            if rock_pos_cache.len() > 100 {
                rock_pos_cache.pop_front();
            }
            rock_pos_cache.push_back(pos);
        }
        if rock_counter == num_of_rocks {
            break;
        }
        rock_counter += 1;
    }
    max_rock
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(solve(TEST_INPUT, 2022), 3068);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(solve(INPUT, 2022), 3098);
    }
}
