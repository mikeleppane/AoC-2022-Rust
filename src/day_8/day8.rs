use nom::combinator::map;
use nom::error::ErrorKind::NoneOf;
use std::cmp::max_by_key;
use std::collections::VecDeque;

const TEST_INPUT: &str = include_str!("./test_input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct GridCoord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub(crate) struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub(crate) fn insert_cell(&mut self, coord: GridCoord, value: T) {
        if !self.in_bounds(coord) {
            panic!(
                "Out of bounds. Cannot insert a value to grid in coordinate {:?}",
                coord
            )
        }
        self.data[coord.y * self.width + coord.x] = value
    }

    pub(crate) fn cell(&self, coord: GridCoord) -> Option<&T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }

    pub(crate) fn height(&self) -> usize {
        self.height
    }
}

const INPUT: &str = include_str!("./input.txt");

fn create_grid_from_input(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            grid.insert_cell((x, y).into(), col as usize - '0' as usize);
        }
    }
    grid
}

pub fn run() {
    println!("=== Day 8 ===");
    println!("Day 8 part 1, solution: {}", part1(INPUT));
    println!("Day 8 part 2, solution: {}", part2(INPUT));
    println!("=============");
    println!();
}

fn part1(input: &str) -> usize {
    let grid = create_grid_from_input(input);
    let all_coords = (0..grid.height()).into_iter().flat_map(|y| {
        (0..grid.width())
            .into_iter()
            .map(move |x| GridCoord::from((x, y)))
    });
    let num_visible_cells = all_coords
        .filter(|&coord| {
            let coord_height = grid.cell(coord).unwrap();
            let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            deltas.iter().any(|&(dx, dy)| {
                let mut cells_in_ine = (1..).into_iter().map_while(|i| {
                    let x = if ((dx * i) + coord.x as isize) < 0 {
                        return None;
                    } else {
                        ((dx * i) + coord.x as isize) as usize
                    };
                    let y = if ((dy * i) + coord.y as isize) < 0 {
                        return None;
                    } else {
                        ((dy * i) + coord.y as isize) as usize
                    };
                    let coord = GridCoord { x, y };
                    grid.cell(coord)
                });
                cells_in_ine.all(|height| height < coord_height)
            })
        })
        .count();
    num_visible_cells
}

fn visible_trees_in_dir(grid: &Grid<usize>, coord: GridCoord, (dx, dy): (isize, isize)) -> usize {
    let line = (1..).into_iter().map_while(|i| {
        let x = if ((dx * i) + coord.x as isize) < 0 {
            return None;
        } else {
            ((dx * i) + coord.x as isize) as usize
        };
        let y = if ((dy * i) + coord.y as isize) < 0 {
            return None;
        } else {
            ((dy * i) + coord.y as isize) as usize
        };
        let coord = GridCoord { x, y };
        Some(*grid.cell(coord)?)
    });

    let mut total = 0;
    let our_height = *grid.cell(coord).unwrap();
    for height in line {
        total += 1;
        if height >= our_height {
            break;
        }
    }
    total
}

fn scenic_score(grid: &Grid<usize>, coord: GridCoord) -> usize {
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    dirs.into_iter()
        .map(|(dx, dy)| visible_trees_in_dir(grid, coord, (dx, dy)))
        .product()
}

fn part2(input: &str) -> usize {
    let grid = create_grid_from_input(input);
    let all_coords = (0..grid.height())
        .into_iter()
        .flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));
    all_coords
        .map(|coord| scenic_score(&grid, coord))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 21);
    }
    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 21);
    }
    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 8);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 8);
    }
}
