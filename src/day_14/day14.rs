use color_eyre::eyre::{Error, Result};
use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");
const INPUT: &str = include_str!("./input.txt");

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) enum Element {
    Air,
    Rock,
    Sand,
}

impl From<&str> for Element {
    fn from(elem: &str) -> Self {
        match elem {
            "#" => Self::Rock,
            "." => Self::Air,
            "+" => Self::Sand,
            _ => panic!("Unrecognized element encoutered => {}", elem),
        }
    }
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

impl FromStr for Coord {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s
            .trim()
            .split_once(',')
            .unwrap_or_else(|| panic!("Could not split str {} ", s));
        let x = x.parse::<usize>()?;
        let y = y.parse::<usize>()?;

        Ok(Self { x, y })
    }
}

fn read_lines(input: &str) -> Result<HashMap<Coord, Element>> {
    let mut coordinates = HashMap::new();
    for line in input.lines() {
        for (p1, p2) in line.split("->").tuple_windows::<(&str, &str)>() {
            let p_start = Coord::from_str(p1)?;
            let p_end = Coord::from_str(p2)?;
            if p_start.x == p_end.x {
                for y in cmp::min(p_start.y, p_end.y)..=cmp::max(p_start.y, p_end.y) {
                    coordinates.insert(Coord::from((p_start.x, y)), Element::from("#"));
                }
            }
            if p_start.y == p_end.y {
                for x in cmp::min(p_start.x, p_end.x)..=cmp::max(p_start.x, p_end.x) {
                    coordinates.insert(Coord::from((x, p_start.y)), Element::from("#"));
                }
            }
        }
    }
    Ok(coordinates)
}

pub fn run() {
    println!("=== Day 14 ===");
    println!("Day 14 part 1, solution: {}", part1(INPUT));
    println!("Day 14 part 2, solution: {}", part2(INPUT));
    println!("=============");
    println!();
}

fn calculate_bottom(grid: &HashMap<Coord, Element>) -> usize {
    grid.iter().max_by_key(|(c, _e)| c.y).unwrap().0.y
}

fn part1(input: &str) -> usize {
    let mut grid = read_lines(input).expect("Something wne wrong while generating grid!");
    let bottom = calculate_bottom(&grid);
    let steps: [(isize, isize); 3] = [(0, 1), (-1, 1), (1, 1)];
    'outer: loop {
        let mut sand = Coord::from((500, 0));
        'inner: loop {
            let mut found = false;
            for step in &steps {
                let sand_pos = sand.get();
                let mut next_coord = Coord::from(sand_pos);
                next_coord.add(*step);
                if !grid.contains_key(&next_coord) {
                    if next_coord.y > bottom {
                        break 'outer;
                    }
                    sand.add(*step);
                    found = true;
                    break;
                }
            }
            if !found {
                grid.insert(sand, Element::Sand);
                break 'inner;
            }
        }
    }

    grid.iter().filter(|(_, v)| **v == Element::Sand).count()
}

fn part2(input: &str) -> usize {
    let mut grid = read_lines(input).expect("Something wne wrong while generating grid!");
    let bottom = calculate_bottom(&grid) + 2;
    let steps: [(isize, isize); 3] = [(0, 1), (-1, 1), (1, 1)];
    'outer: loop {
        let mut sand = Coord::from((500, 0));
        'inner: loop {
            let mut found = false;
            for step in &steps {
                let sand_pos = sand.get();
                let mut next_coord = Coord::from(sand_pos);
                next_coord.add(*step);
                if !grid.contains_key(&next_coord) {
                    sand.add(*step);
                    if next_coord.y == bottom - 1 {
                        grid.insert(sand, Element::Sand);
                        break 'inner;
                    }
                    found = true;
                    break;
                }
            }
            if sand.get() == (500, 0) {
                grid.insert(sand, Element::Sand);
                break 'outer;
            }
            if !found {
                grid.insert(sand, Element::Sand);
                break 'inner;
            }
        }
    }

    grid.iter().filter(|(_, v)| **v == Element::Sand).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 24);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 897);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 93);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 26683);
    }
}
