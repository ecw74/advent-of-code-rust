use super::day04::Day04;

impl Day04 {
    /// Parses the input string into a 2D matrix of characters.
    /// Each line in the input becomes a row in the matrix, ignoring empty lines.
    pub fn parse_input(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines
            .map(|line| line.chars().collect()) // Convert each line into a vector of characters
            .collect() // Collect all lines into a 2D vector
    }

    /// Counts all occurrences of the word "XMAS" in the matrix, in any direction.
    pub fn count_xmas_words(matrix: &[Vec<char>]) -> usize {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut sum_extracted_xmas = 0;

        // "XMAS" as a constant sequence of characters
        const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

        /// Checks if "XMAS" exists in the specified direction (dr, dc) starting from (row, col).
        fn check_direction(
            matrix: &[Vec<char>],
            rows: usize,
            cols: usize,
            row: usize,
            col: usize,
            dr: isize,
            dc: isize,
        ) -> bool {
            let mut buffer = ['\0'; 4]; // Temporary buffer for the characters being checked
            for i in 0..4 {
                let nr = row as isize + i * dr; // Calculate the row index for the current character
                let nc = col as isize + i * dc; // Calculate the column index for the current character

                // Ensure indices are within bounds
                if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                    return false;
                }

                // Collect the character from the matrix into the buffer
                buffer[i as usize] = matrix[nr as usize][nc as usize];
            }

            buffer == XMAS // Check if the buffer matches "XMAS"
        }

        // Traverse the entire matrix
        for r in 0..rows {
            for c in 0..cols {
                // All possible directions for "XMAS"
                let directions = [
                    (-1, -1), // Up-left
                    (-1, 0),  // Up
                    (-1, 1),  // Up-right
                    (0, -1),  // Left
                    (0, 1),   // Right
                    (1, -1),  // Down-left
                    (1, 0),   // Down
                    (1, 1),   // Down-right
                ];

                // Check each direction for "XMAS"
                for &(dr, dc) in &directions {
                    if check_direction(matrix, rows, cols, r, c, dr, dc) {
                        sum_extracted_xmas += 1; // Increment count if "XMAS" is found
                    }
                }
            }
        }

        sum_extracted_xmas // Return the total count
    }

    /// Solves part 1 of the puzzle: counts all occurrences of "XMAS" in the input.
    pub fn part_1(&self, input: &str) -> String {
        let matrix = Self::parse_input(input); // Parse the input into a matrix
        let sum = Self::count_xmas_words(&matrix); // Count all "XMAS" occurrences
        sum.to_string() // Convert the result to a string
    }

    /// Counts occurrences of the "X-MAS" pattern in the matrix.
    pub fn find_x_mas(matrix: &[Vec<char>]) -> usize {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut sum_mas = 0;

        // Traverse the matrix, excluding edges (since X-MAS requires neighbors)
        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                // Check if the current cell is 'A', the center of the X-MAS pattern
                if matrix[r][c] == 'A' {
                    // Check for all valid X-MAS configurations
                    if (matrix[r - 1][c - 1] == 'M' // Top-left
                        && matrix[r + 1][c + 1] == 'S' // Bottom-right
                        && matrix[r + 1][c - 1] == 'M' // Bottom-left
                        && matrix[r - 1][c + 1] == 'S') // Top-right
                        || (matrix[r - 1][c - 1] == 'S'
                        && matrix[r + 1][c + 1] == 'M'
                        && matrix[r + 1][c - 1] == 'S'
                        && matrix[r - 1][c + 1] == 'M')
                        || (matrix[r - 1][c - 1] == 'M'
                        && matrix[r + 1][c + 1] == 'S'
                        && matrix[r + 1][c - 1] == 'S'
                        && matrix[r - 1][c + 1] == 'M')
                        || (matrix[r - 1][c - 1] == 'S'
                        && matrix[r + 1][c + 1] == 'M'
                        && matrix[r + 1][c - 1] == 'M'
                        && matrix[r - 1][c + 1] == 'S')
                    {
                        sum_mas += 1; // Increment count if a valid X-MAS pattern is found
                    }
                }
            }
        }

        sum_mas // Return the total count
    }

    /// Solves part 2 of the puzzle: counts all occurrences of the "X-MAS" pattern.
    pub fn part_2(&self, input: &str) -> String {
        let matrix = Self::parse_input(input); // Parse the input into a matrix
        let sum = Self::find_x_mas(&matrix); // Count all "X-MAS" occurrences
        sum.to_string() // Convert the result to a string
    }
}

mod test {
    #[test]
    /// Test for part 1: verifies correct "XMAS" counting in the example input.
    fn test_aoc2024_day04_part_1() {
        use crate::solutions::aoc2024::Day04;
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        "#;
        assert_eq!(day04.part_1(input), "18");
    }

    #[test]
    /// Test for part 2: verifies correct "X-MAS" pattern counting in the example input.
    fn test_aoc2024_day04_part_2() {
        use crate::solutions::aoc2024::Day04;
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        "#;
        assert_eq!(day04.part_2(input), "9");
    }
}
