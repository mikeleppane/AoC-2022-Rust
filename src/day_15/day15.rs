use std::fmt;

use nom::{
    bytes::streaming::tag,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn parse(i: &str) -> IResult<&str, Point> {
        map(
            separated_pair(
                preceded(tag("x="), nom::character::complete::i64),
                tag(", "),
                preceded(tag("y="), nom::character::complete::i64),
            ),
            |(x, y)| Point { x, y },
        )(i)
    }

    pub fn manhattan_dist(self, other: Self) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}

#[derive(Debug)]
pub struct Record {
    pub sensor: Point,
    pub beacon: Point,
}

impl Record {
    pub fn must_parse(i: &str) -> Self {
        all_consuming(Self::parse)(i)
            .finish()
            .expect("failed to parse input")
            .1
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("Sensor at "), Point::parse),
                tag(": closest beacon is at "),
                Point::parse,
            ),
            |(sensor, beacon)| Record { sensor, beacon },
        )(i)
    }
}

struct Map {
    records: Vec<Record>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let records = input.lines().map(Record::must_parse).collect();
        Self { records }
    }

    fn dump(&self) {
        for record in &self.records {
            println!("{record:?}");
        }
    }

    fn num_impossible_positions(&self, y: i64) -> usize {
        let mut total = 0;
        let min_x = -4;
        let max_x = 26;

        for x in min_x..=max_x {
            let point = Point { x, y };
            if self.records.iter().any(|rec| rec.beacon == point) {
                // already have a beacon there, not an impossible position
            } else if self.records.iter().any(|rec| {
                let radius = rec.sensor.manhattan_dist(rec.beacon);
                rec.sensor.manhattan_dist(point) <= radius
            }) {
                // covered!
                total += 1
            }
        }

        total
}

fn part1(input: &str) -> u64 {
    let map = Map::parse(input);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT, 2000000), 26);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 26);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 26);
    }
}
