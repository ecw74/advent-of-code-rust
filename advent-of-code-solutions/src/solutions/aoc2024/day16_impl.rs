use super::day16::Day16;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    position: (usize, usize),
    direction: char,
    cost: usize,
}

impl State {
    fn new(x: usize, y: usize, dir: char, cost: usize) -> Self {
        State {
            position: (x, y),
            direction: dir,
            cost,
        }
    }
}

// Directions and their deltas
const DIRECTIONS: [(char, (isize, isize)); 4] = [
    ('N', (-1, 0)), // North
    ('E', (0, 1)),  // East
    ('S', (1, 0)),  // South
    ('W', (0, -1)), // West
];

impl Day16 {
    fn parse_map(input: &str) -> (Vec<Vec<char>>, (usize, usize, char), (usize, usize)) {
        let mut map = Vec::new();
        let mut start = (0, 0, 'E');
        let mut end = (0, 0);

        for (row_idx, line) in input.lines().enumerate() {
            let line = line.trim(); // Remove leading and trailing whitespace.
            if line.is_empty() {
                continue; // Ignore empty lines.
            }

            let mut row = Vec::new();
            for (col_idx, ch) in line.chars().enumerate() {
                match ch {
                    'S' => start = (row_idx, col_idx, 'E'),
                    'E' => end = (row_idx, col_idx),
                    '#' | '.' => {} // Valid map characters, no action needed.
                    _ => panic!(
                        "Invalid character '{}' in map at ({}, {})",
                        ch, row_idx, col_idx
                    ),
                }
                row.push(ch);
            }
            map.push(row);
        }

        (map, start, end)
    }

    fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
        (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
    }

    fn a_star(
        map: &Vec<Vec<char>>,
        start: (usize, usize, char),
        end: (usize, usize),
    ) -> Option<usize> {
        let mut open_set = BinaryHeap::new();
        let mut g_score: HashMap<(usize, usize, char), usize> = HashMap::new();

        // Initial state
        let start_state = State::new(start.0, start.1, start.2, 0);
        open_set.push(Reverse((0, start_state.clone())));
        g_score.insert((start.0, start.1, start.2), 0);

        while let Some(Reverse((_, current))) = open_set.pop() {
            if current.position == end {
                return Some(current.cost);
            }

            for &(new_dir, (dx, dy)) in DIRECTIONS.iter() {
                let new_x = current.position.0 as isize + dx;
                let new_y = current.position.1 as isize + dy;

                // Check map boundaries
                if new_x < 0
                    || new_y < 0
                    || new_x as usize >= map.len()
                    || new_y as usize >= map[0].len()
                {
                    continue;
                }

                let new_pos = (new_x as usize, new_y as usize);
                if map[new_pos.0][new_pos.1] == '#' {
                    continue;
                }

                // Calculate cost
                let turn_cost = if current.direction == new_dir {
                    0
                } else {
                    1000
                };
                let move_cost = 1;
                let total_cost = current.cost + turn_cost + move_cost;

                // If the new cost is better, update it
                let state_key = (new_pos.0, new_pos.1, new_dir);
                if total_cost < *g_score.get(&state_key).unwrap_or(&usize::MAX) {
                    g_score.insert(state_key, total_cost);
                    let priority = total_cost + Self::heuristic(new_pos, end);
                    open_set.push(Reverse((
                        priority,
                        State::new(new_pos.0, new_pos.1, new_dir, total_cost),
                    )));
                }
            }
        }

        None // If no path is found
    }

    pub fn part_1(&self, input: &str) -> String {
        let (map, start, end) = Self::parse_map(input);
        let costs = Self::a_star(&map, start, end);
        costs.unwrap_or(0).to_string()
    }
    pub fn part_2(&self, _input: &str) -> String {
        todo!()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day16 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day16_part_1() {
        use crate::solutions::aoc2024::Day16;
        let day16 = Day16 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        "#;
        assert_eq!(day16.part_1(input), "7036"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day16 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day16_part_2() {
        use crate::solutions::aoc2024::Day16;
        let day16 = Day16 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        assert_eq!(day16.part_2(input), "TBD"); // Asserts if the function output matches the expected result.
    }
}
