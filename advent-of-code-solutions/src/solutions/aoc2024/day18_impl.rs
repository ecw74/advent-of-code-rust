use super::day18::Day18;
use pathfinding::prelude::{bfs, Grid};

impl Day18 {
    /// Parses the input string into a vector of `(usize, usize)` tuples, representing the coordinates
    /// of falling bytes as specified in the input. Each line of the input corresponds to one coordinate pair.
    fn parse_coordinates_to_grid(input: &str) -> Vec<(usize, usize)> {
        input
            .lines()
            .filter(|s| !s.trim().is_empty()) // Ignore empty lines.
            .filter_map(|line| line.trim().split_once(',')) // Split each line by ',' to separate x and y.
            .filter_map(|(x, y)| Some((x.parse::<usize>().ok()?, y.parse::<usize>().ok()?))) // Parse x and y as usize.
            .collect()
    }

    /// Solves part 1 of the problem:
    /// Calculates the minimum number of steps required to move from the top-left corner `(0,0)`
    /// to the bottom-right corner `(6,6)` or `(70,70)` based on the grid size, simulating corrupted bytes.
    pub fn part_1(&self, input: &str) -> String {
        let bytes;
        let coords = Self::parse_coordinates_to_grid(input); // Parse falling bytes from input.
        let start = (0usize, 0usize); // Starting position at the top-left corner.
        let goal;

        // Determine grid size and how many bytes to simulate based on input length.
        if input.len() < 1024 {
            goal = (6, 6); // Example case for smaller input.
            bytes = coords.iter().take(12).cloned(); // Simulate first 12 bytes.
        } else {
            goal = (70, 70); // Larger grid for full input.
            bytes = coords.iter().take(1024).cloned(); // Simulate first 1024 bytes.
        }

        let mut grid = Grid::new(goal.0 + 1, goal.1 + 1); // Create a grid with size (goal.x + 1, goal.y + 1).
        grid.fill(); // Initially, mark all positions as accessible.

        // Mark positions corrupted by falling bytes.
        for byte in bytes {
            grid.remove_vertex(byte);
        }

        // Define a closure to calculate valid neighbors for BFS traversal.
        let neighbors = |&(x, y): &(usize, usize)| -> Vec<(usize, usize)> {
            grid.neighbours((x, y)) // Use the grid's helper to find accessible neighbors.
        };

        // Use BFS to find the shortest path from the start to the goal.
        let path = bfs(&start, neighbors, |&p| p == goal).unwrap();

        (path.len() - 1).to_string() // Return the path length minus one (steps count).
    }

    /// Solves part 2 of the problem:
    /// Determines the first byte that makes the exit unreachable from the starting point.
    pub fn part_2(&self, input: &str) -> String {
        let bytes = Self::parse_coordinates_to_grid(input); // Parse falling bytes from input.
        let start = (0usize, 0usize); // Starting position at the top-left corner.
        let goal;

        // Determine the goal based on input length.
        if input.len() < 1024 {
            goal = (6, 6); // Example case for smaller input.
        } else {
            goal = (70, 70); // Larger grid for full input.
        }

        let mut grid = Grid::new(goal.0 + 1, goal.1 + 1); // Create the grid.

        let mut possible = 0; // Index of the last byte where the path was still reachable.
        let mut impossible = bytes.len() - 1; // Index of the first byte where the path becomes unreachable.

        // Binary search to find the critical byte that blocks the path.
        while impossible > (possible + 1) {
            let midpoint = (possible + impossible) / 2; // Check the midpoint of the range.
            let bytes = bytes.iter().take(midpoint).collect::<Vec<_>>(); // Simulate bytes up to midpoint.

            grid.fill(); // Reset the grid to be fully accessible.

            // Mark positions corrupted by falling bytes.
            for byte in bytes {
                grid.remove_vertex(*byte);
            }

            // Define a closure to calculate valid neighbors for BFS traversal.
            let neighbors =
                |&(x, y): &(usize, usize)| -> Vec<(usize, usize)> { grid.neighbours((x, y)) };

            // Check if a path from start to goal exists with the current set of bytes.
            if let Some(_) = bfs(&start, neighbors, |&p| p == goal) {
                possible = midpoint; // Path exists; move `possible` forward.
            } else {
                impossible = midpoint; // Path blocked; move `impossible` back.
            }
        }
        format!("{},{}", bytes[possible].0, bytes[possible].1) // Return the critical byte coordinates.
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day 18 for Advent of Code 2024.
    /// Verifies the correct number of steps from start to goal with initial corrupted bytes.
    fn test_aoc2024_day18_part_1() {
        use crate::solutions::aoc2024::Day18;
        let day18 = Day18 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
        "#;
        assert_eq!(day18.part_1(input), "22"); // Expected result: 22 steps.
    }

    #[test]
    /// Test for part 2 of Day 18 for Advent of Code 2024.
    /// Verifies the correct detection of the first critical byte that blocks the path to the goal.
    fn test_aoc2024_day18_part_2() {
        use crate::solutions::aoc2024::Day18;
        let day18 = Day18 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
        "#;
        assert_eq!(day18.part_2(input), "6,1"); // Expected result: byte at (6,1).
    }
}
