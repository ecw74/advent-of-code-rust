use std::collections::HashSet;
use super::day10::Day10;

impl Day10 {
    // Parses the input string into a 2D grid of u8 values representing the map.
    fn parse_to_grid(input: &str) -> Vec<Vec<u8>> {
        input
            .lines()
            .filter(|s| !s.trim().is_empty()) // Ignore empty lines
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8) // Convert each character to a digit
                    .collect() // Collect into a row of u8 values
            })
            .collect() // Collect all rows into the grid
    }

    // Helper function to find all trailheads (positions with height 0) in the grid.
    fn find_trailheads(grid: &[Vec<u8>]) -> Vec<(u8, u8)> {
        grid.iter()
            .enumerate() // Iterate over rows with indices
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate() // Iterate over columns with indices
                    .filter_map(move |(col, &value)| {
                        if value == 0 {
                            Some((row as u8, col as u8)) // Collect coordinates of height-0 cells
                        } else {
                            None // Skip non-zero cells
                        }
                    })
            })
            .collect() // Collect all trailhead positions
    }

    // Finds all valid neighboring positions for a given cell in the grid.
    fn neighbors(row: u8, col: u8, rows: u8, cols: u8) -> Vec<(u8, u8)> {
        let mut result = Vec::new();
        if row > 0 {
            result.push((row - 1, col)); // Add the cell above
        }
        if row + 1 < rows {
            result.push((row + 1, col)); // Add the cell below
        }
        if col > 0 {
            result.push((row, col - 1)); // Add the cell to the left
        }
        if col + 1 < cols {
            result.push((row, col + 1)); // Add the cell to the right
        }
        result // Return the list of neighbors
    }

    // Recursive DFS helper function for exploring trails in both parts.
    fn explore_trail(
        grid: &[Vec<u8>],            // The grid to traverse
        row: u8,                     // Current row position
        col: u8,                     // Current column position
        visited: &mut HashSet<(u8, u8)>, // Set to track visited cells
        score_accumulator: &mut usize,  // Counter to track scores or trail counts
        count_score: bool,              // Flag: true for Part 1, false for Part 2
    ) {
        if count_score {
            // Skip this cell if it has already been visited
            if visited.contains(&(row, col)) {
                return;
            }
        }

        visited.insert((row, col)); // Mark the cell as visited

        // If the current cell is height `9`, increase the score or trail count
        if grid[row as usize][col as usize] == 9 {
            *score_accumulator += 1;
            if count_score {
                return; // Stop further traversal for Part 1
            }
        }

        // Recursively explore neighbors that satisfy the hiking trail rules
        for (next_row, next_col) in Self::neighbors(row, col, grid.len() as u8, grid[0].len() as u8) {
            if !visited.contains(&(next_row, next_col))
                && grid[next_row as usize][next_col as usize] == grid[row as usize][col as usize] + 1
            {
                Self::explore_trail(grid, next_row, next_col, visited, score_accumulator, count_score);
            }
        }

        visited.remove(&(row, col)); // Backtrack: unmark the cell as visited
    }

    // Part 1: Calculate the total score of all trailheads.
    pub fn part_1(&self, input: &str) -> String {
        let grid = Self::parse_to_grid(input); // Parse the input into a grid
        let mut total_score = 0;

        let trailheads = Self::find_trailheads(&grid); // Find all trailhead positions

        for (row, col) in trailheads {
            let mut visited = HashSet::new(); // Initialize the visited set
            let mut score = 0; // Initialize the score counter
            Self::explore_trail(&grid, row, col, &mut visited, &mut score, true); // Explore trails
            total_score += score; // Add the score for this trailhead
        }

        total_score.to_string() // Return the total score as a string
    }

    // Part 2: Calculate the total number of distinct hiking trails.
    pub fn part_2(&self, input: &str) -> String {
        let grid = Self::parse_to_grid(input); // Parse the input into a grid
        let mut total_distinct_trails = 0;

        let trailheads = Self::find_trailheads(&grid); // Find all trailhead positions

        for (row, col) in trailheads {
            let mut visited = HashSet::new(); // Initialize the visited set
            let mut distinct_trails = 0; // Initialize the trail count
            Self::explore_trail(&grid, row, col, &mut visited, &mut distinct_trails, false); // Explore trails
            total_distinct_trails += distinct_trails; // Add the trail count for this trailhead
        }

        total_distinct_trails.to_string() // Return the total trail count as a string
    }
}

mod test {
    #[test]
    /// Test for Part 1: Ensures correct calculation of the total score of trailheads.
    fn test_aoc2024_day10_part_1() {
        use crate::solutions::aoc2024::Day10;
        let day10 = Day10 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "#;
        assert_eq!(day10.part_1(input), "36"); // Expected output: "36"
    }

    #[test]
    /// Test for Part 2: Verifies correct calculation of distinct hiking trails.
    fn test_aoc2024_day10_part_2() {
        use crate::solutions::aoc2024::Day10;
        let day10 = Day10 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "#;
        assert_eq!(day10.part_2(input), "81"); // Expected output: "81"
    }
}
