use super::day16::Day16;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use num::abs;

/// Represents a state in the maze pathfinding process.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct State {
    position: (u8, u8),  // Current position in the maze as (row, col).
    direction: char,           // Current direction ('N', 'E', 'S', 'W').
    cost: usize,               // Accumulated cost to reach this state.
    path: Vec<(u8, u8)>, // Path taken to reach this state.
}

impl State {
    /// Creates a new State instance.
    fn new(x: u8, y: u8, dir: char, cost: usize, path: Vec<(u8, u8)>) -> Self {
        State {
            position: (x, y),
            direction: dir,
            cost,
            path,
        }
    }
}

// Definitions for cardinal directions and their corresponding coordinate deltas.
const DIRECTIONS: [(char, (isize, isize)); 4] = [
    ('N', (-1, 0)), // North: move up.
    ('E', (0, 1)),  // East: move right.
    ('S', (1, 0)),  // South: move down.
    ('W', (0, -1)), // West: move left.
];

impl Day16 {
    /// Parses the maze input and extracts the map, start position, and end position.
    fn parse_map(input: &str) -> (Vec<Vec<char>>, (u8, u8, char), (u8, u8)) {
        let mut map = Vec::new(); // The maze represented as a 2D grid.
        let mut start = (0, 0, 'E'); // Default start position facing East.
        let mut end = (0, 0); // Default end position.

        for (row_idx, line) in input.lines().enumerate() {
            let line = line.trim(); // Remove any whitespace around the line.
            if line.is_empty() {
                continue; // Skip empty lines.
            }

            let mut row = Vec::new();
            for (col_idx, ch) in line.chars().enumerate() {
                match ch {
                    'S' => start = (row_idx as u8, col_idx as u8, 'E'), // Record the start position.
                    'E' => end = (row_idx as u8, col_idx as u8),        // Record the end position.
                    '#' | '.' => {} // Valid map characters, but no special handling required.
                    _ => panic!(
                        "Invalid character '{}' in map at ({}, {})",
                        ch, row_idx, col_idx
                    ),
                }
                row.push(ch); // Add the character to the current row.
            }
            map.push(row); // Add the row to the map.
        }

        (map, start, end) // Return the parsed map, start, and end positions.
    }

    /// Heuristic function for A*: Manhattan distance between two positions.
    fn heuristic(a: (u8, u8), b: (u8, u8)) -> usize {
        (abs(a.0 as i32 - b.0 as i32) + abs(a.1 as i32 - b.1 as i32)) as usize
    }

    /// Executes the maze pathfinding and determines the lowest cost and optimal tiles.
    fn run_maze(
        map: &Vec<Vec<char>>,        // The maze map.
        start: (u8, u8, char), // Start position with direction.
        end: (u8, u8),         // End position.
    ) -> (usize, usize) {
        // Returns (lowest cost, number of optimal tiles).
        let mut open_set = BinaryHeap::new(); // Priority queue for A* exploration.
        let mut g_score: HashMap<(u8, u8, char), usize> = HashMap::new(); // Best-known costs.
        let mut final_cost = None; // Lowest cost to reach the end.
        let mut best_seats: HashSet<(u8, u8)> = HashSet::default(); // Tiles in optimal paths.

        // Initialize the starting state.
        let start_state = State::new(start.0, start.1, start.2, 0, vec![(start.0, start.1)]);
        open_set.push(Reverse((0, start_state.clone()))); // Push the start state with priority 0.
        g_score.insert((start.0, start.1, start.2), 0); // Initial cost is 0.

        while let Some(Reverse((_, current))) = open_set.pop() {
            // If the current position is the end, process the path.
            if current.position == end {
                if final_cost.is_none() {
                    final_cost = Some(current.cost); // Record the first time we reach the end.
                }
                if final_cost == Some(current.cost) {
                    best_seats.extend(current.path.into_iter()); // Mark the tiles in the path.
                }
                continue; // Continue exploring other potential optimal paths.
            }

            // Explore all possible directions.
            for &(new_dir, (dx, dy)) in DIRECTIONS.iter() {
                let new_x = current.position.0 as isize + dx;
                let new_y = current.position.1 as isize + dy;

                // Check boundaries and walls.
                if new_x < 0
                    || new_y < 0
                    || new_x as usize >= map.len()
                    || new_y as usize >= map[0].len()
                {
                    continue; // Skip positions out of bounds.
                }

                let new_pos = (new_x as u8, new_y as u8);
                if map[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                    continue; // Skip walls.
                }

                // Calculate movement and turn costs.
                let turn_cost = if current.direction == new_dir {
                    0 // No cost if moving in the same direction.
                } else {
                    1000 // 1000 points for a 90-degree turn.
                };

                let move_cost = 1; // 1 point for moving forward.
                let total_cost = current.cost + turn_cost + move_cost;

                let mut path = current.path.clone(); // Clone the current path.
                path.push(new_pos); // Add the new position to the path.

                // Update state if the new cost is better.
                let state_key = (new_pos.0, new_pos.1, new_dir);
                if total_cost <= *g_score.get(&state_key).unwrap_or(&usize::MAX) {
                    g_score.insert(state_key, total_cost); // Update best cost.
                    let priority = total_cost + Self::heuristic(new_pos, end); // Priority with heuristic.
                    open_set.push(Reverse((
                        priority,
                        State::new(new_pos.0, new_pos.1, new_dir, total_cost, path),
                    )));
                }
            }
        }

        (final_cost.unwrap(), best_seats.len()) // Return the lowest cost and the count of optimal tiles.
    }

    /// Solves Part 1: Calculate the lowest cost to traverse the maze.
    pub fn part_1(&self, input: &str) -> String {
        let (map, start, end) = Self::parse_map(input);
        let maze = Self::run_maze(&map, start, end);
        maze.0.to_string() // Return the lowest cost as a string.
    }

    /// Solves Part 2: Determine the number of tiles in the optimal paths.
    pub fn part_2(&self, input: &str) -> String {
        let (map, start, end) = Self::parse_map(input);
        let maze = Self::run_maze(&map, start, end);
        maze.1.to_string() // Return the count of optimal tiles as a string.
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
        assert_eq!(day16.part_2(input), "45"); // Asserts if the function output matches the expected result.
    }
}
