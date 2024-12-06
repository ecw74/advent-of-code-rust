use super::day06::Day06;
use std::collections::HashSet;

/// Struct to store the map details, including the guard's starting position,
/// map size, and the locations of obstacles (`#`).
#[derive(Debug)]
struct MapData {
    start_position: ((u8, u8), char), // ((x, y), direction)
    size: (u8, u8),                   // Dimensions of the map (width, height)
    obstacles: HashSet<(u8, u8)>,     // Set of obstacle coordinates
}

impl Day06 {
    /// Parses the input map into a `MapData` structure.
    /// Extracts the guard's starting position, map size, and obstacle positions.
    fn parse_map(input: &str) -> MapData {
        let mut start_position = None; // Placeholder for guard's start position
        let mut obstacles = HashSet::new();

        // Remove empty lines and prepare the input for parsing
        let filtered_input: Vec<&str> = input
            .lines()
            .map(|line| line.trim()) // Trim whitespace from each line
            .filter(|line| !line.is_empty()) // Skip empty lines
            .collect();
        let y_size = filtered_input.len() as u8; // Number of rows
        let x_size = filtered_input.get(0).map_or(0, |row| row.len()) as u8; // Number of columns

        // Iterate through the map and populate start position and obstacle set
        for (y, row) in filtered_input.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    '^' | '>' | '<' | 'v' => {
                        start_position = Some(((x as u8, y as u8), ch)) // Guard's starting point
                    }
                    '#' => {
                        obstacles.insert((x as u8, y as u8)); // Add obstacle position
                    }
                    _ => {}
                }
            }
        }

        // Ensure the guard's starting position was found in the input
        MapData {
            start_position: start_position.expect("No start position found in the map!"),
            size: (x_size, y_size),
            obstacles,
        }
    }

    /// Rotates the guard's direction 90 degrees to the right.
    fn rotate_right(direction: char) -> char {
        match direction {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!("Invalid direction!"), // Should never occur
        }
    }

    /// Simulates the guard's movement across the map.
    ///
    /// - `start_position`: The initial position and direction of the guard.
    /// - `obstacles`: Set of obstacle positions.
    /// - `size`: Dimensions of the map (width, height).
    /// - `distinct_positions`: Tracks all visited positions (mutated in place).
    /// - `detect_loop`: If true, detects loops instead of recording distinct positions.
    ///
    /// Returns `true` if a loop is detected, otherwise `false`.
    fn simulate_steps(
        start_position: &((u8, u8), char),
        obstacles: &HashSet<(u8, u8)>,
        size: &(u8, u8),
        distinct_positions: &mut HashSet<(u8, u8)>,
        detect_loop: bool,
    ) -> bool {
        let mut position = start_position.0; // Current position (x, y)
        let mut direction = start_position.1; // Current direction ('^', '>', '<', 'v')
        let x_size = size.0; // Map width
        let y_size = size.1; // Map height
        let mut visited = HashSet::new(); // Tracks visited states for loop detection

        // Helper to calculate the next position based on direction
        let next_position = |pos: (u8, u8), dir: char| -> (i16, i16) {
            match dir {
                '^' => (pos.0 as i16, pos.1 as i16 - 1), // Move up
                '>' => (pos.0 as i16 + 1, pos.1 as i16), // Move right
                'v' => (pos.0 as i16, pos.1 as i16 + 1), // Move down
                '<' => (pos.0 as i16 - 1, pos.1 as i16), // Move left
                _ => (pos.0 as i16, pos.1 as i16),       // No movement
            }
        };

        // Main loop to simulate the guard's patrol
        loop {
            let new_position = next_position(position, direction);
            // Check if the guard leaves the map boundaries
            if new_position.0 < 0
                || new_position.1 < 0
                || new_position.0 >= x_size as i16
                || new_position.1 >= y_size as i16
            {
                return false; // Guard has exited the map
            }
            let new_position = (new_position.0 as u8, new_position.1 as u8);

            // Check if the next position is an obstacle
            if obstacles.contains(&new_position) {
                if detect_loop && visited.contains(&(new_position, direction)) {
                    return true; // Loop detected
                }
                visited.insert((new_position, direction)); // Mark state as visited
                direction = Self::rotate_right(direction); // Turn right
            } else {
                position = new_position; // Move forward
                if !detect_loop {
                    distinct_positions.insert(position); // Record visited position
                }
            }
        }
    }

    /// Solves Part 1 of the problem: calculate the number of distinct positions
    /// visited by the guard before leaving the map.
    pub fn part_1(&self, input: &str) -> String {
        let map_data = Self::parse_map(input); // Parse input map
        let mut distinct_positions: HashSet<(u8, u8)> = HashSet::new();

        // Initialize simulation with the guard's starting position
        distinct_positions.insert(map_data.start_position.0);
        Self::simulate_steps(
            &map_data.start_position,
            &map_data.obstacles,
            &map_data.size,
            &mut distinct_positions,
            false, // Not detecting loops
        );
        distinct_positions.len().to_string()
    }

    /// Solves Part 2 of the problem: find the number of positions where adding
    /// an obstacle would trap the guard in a loop.
    pub fn part_2(&self, input: &str) -> String {
        let mut map_data = Self::parse_map(input); // Parse input map
        let mut distinct_positions: HashSet<(u8, u8)> = HashSet::new();

        // Record positions visited without detecting loops
        distinct_positions.insert(map_data.start_position.0);
        Self::simulate_steps(
            &map_data.start_position,
            &map_data.obstacles,
            &map_data.size,
            &mut distinct_positions,
            false,
        );

        // Check each distinct position as a potential obstruction
        let result: usize = distinct_positions
            .iter()
            .filter(|&cor| {
                let added = map_data.obstacles.insert(*cor); // Temporarily add obstacle
                let loop_detected = Self::simulate_steps(
                    &map_data.start_position,
                    &map_data.obstacles,
                    &map_data.size,
                    &mut HashSet::new(),
                    true, // Detect loops
                );
                if added {
                    map_data.obstacles.remove(cor); // Remove temporary obstacle
                }
                loop_detected
            })
            .count();

        result.to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day06 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day06_part_1() {
        use crate::solutions::aoc2024::Day06;
        let day06 = Day06 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "#;
        assert_eq!(day06.part_1(input), "41"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day06 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day06_part_2() {
        use crate::solutions::aoc2024::Day06;
        let day06 = Day06 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "#;
        assert_eq!(day06.part_2(input), "6"); // Asserts if the function output matches the expected result.
    }
}
