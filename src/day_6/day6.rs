use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test_input.txt");

fn read_input(input: &str) -> Vec<char> {
    input.chars().collect::<Vec<char>>()
}

pub fn run() {
    println!("=== Day 6 ===");
    println!("Day 6 part 1, solution: {}", solve(INPUT, 4));
    println!("Day 6 part 2, solution: {}", solve(INPUT, 14));
    println!("=============");
    println!();
}

fn solve(input: &str, window_size: usize) -> usize {
    let datastream = read_input(input);
    let mut marker = 0usize;
    for (index, stream) in datastream.windows(window_size).enumerate() {
        let hash: HashSet<char> = stream.iter().copied().collect();
        if hash.len() == window_size {
            marker = index + window_size;
            break;
        }
    }
    marker
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve(input, 4), 7);
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve(input, 4), 5);
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solve(input, 4), 6);
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solve(input, 4), 10);
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solve(input, 4), 11);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(solve(INPUT, 4), 1142);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(solve(INPUT, 14), 2803);
    }
}
