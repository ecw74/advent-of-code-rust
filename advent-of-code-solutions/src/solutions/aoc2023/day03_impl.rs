use super::day03::Day03;

impl Day03 {
    // Parse the input into a 2D grid
    pub fn parse_input(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.chars().collect())
            .collect()
    }

    // Extract numbers adjacent to symbols
    pub fn extract_numbers_adjacent_to_symbols(matrix: &[Vec<char>]) -> i64 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut sum_extracted_numbers = 0i64;

        // Check if a character is a digit
        let is_digit = |ch: char| ch.is_ascii_digit();

        // Check if a cell is a symbol
        let is_symbol = |ch: char| !ch.is_ascii_digit() && ch != '.';

        // Check adjacency to symbols
        let is_adjacent_to_symbol = |r: usize, c: usize| {
            let directions = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            directions.iter().any(|&(dr, dc)| {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                nr >= 0
                    && nc >= 0
                    && nr < rows as isize
                    && nc < cols as isize
                    && is_symbol(matrix[nr as usize][nc as usize])
            })
        };

        // Iterate through the matrix
        for r in 0..rows {
            let mut c = 0;
            while c < cols {
                if is_digit(matrix[r][c]) {
                    // Start building a number
                    let mut number = 0;
                    let start_c = c;

                    while c < cols && is_digit(matrix[r][c]) {
                        number = number * 10 + (matrix[r][c] as i64 - '0' as i64);
                        c += 1;
                    }

                    // Check if any part of the number is adjacent to a symbol
                    let number_len = c - start_c;
                    for offset in 0..number_len {
                        if is_adjacent_to_symbol(r, start_c + offset) {
                            sum_extracted_numbers += number;
                            break;
                        }
                    }
                } else {
                    c += 1;
                }
            }
        }

        sum_extracted_numbers
    }

    // Part 1: Solve the puzzle
    pub fn part_1(&self, input: &str) -> String {
        let matrix = Self::parse_input(input);
        let sum = Self::extract_numbers_adjacent_to_symbols(&matrix);
        sum.to_string() // Convert the final result to a String
    }

    // Extract gear ratios and sum them
    pub fn find_gear_ratios(matrix: &[Vec<char>]) -> i64 {
        let rows = matrix.len();
        let cols = matrix[0].len();

        // Helper to parse a number starting at a position and direction
        let parse_number_at = |matrix: &[Vec<char>], row: usize, col: usize| -> Option<i64> {
            let rows = matrix.len();
            let cols = matrix[0].len();
            let mut number = String::new();

            // Check left
            if col > 0 && matrix[row][col - 1].is_ascii_digit() {
                let mut i = col as isize - 1;
                while i >= 0 && matrix[row][i as usize].is_ascii_digit() {
                    number.insert(0, matrix[row][i as usize]); // Prepend to the number
                    i -= 1;
                }
            }

            // Check up
            if row > 0 && matrix[row - 1][col].is_ascii_digit() {
                let mut i = row as isize - 1;
                while i >= 0 && matrix[i as usize][col].is_ascii_digit() {
                    number.insert(0, matrix[i as usize][col]); // Prepend to the number
                    i -= 1;
                }
            }

            // Include current digit
            if matrix[row][col].is_ascii_digit() {
                number.push(matrix[row][col]);
            } else {
                return None; // If the current position is not a digit, return None
            }

            // Check right
            let mut i = col + 1;
            while i < cols && matrix[row][i].is_ascii_digit() {
                number.push(matrix[row][i]);
                i += 1;
            }

            // Check down
            let mut i = row + 1;
            while i < rows && matrix[i][col].is_ascii_digit() {
                number.push(matrix[i][col]);
                i += 1;
            }

            number.parse::<i64>().ok()
        };

        // Directions for adjacency
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1), // Top-left, Top, Top-right
            (0, -1),
            (0, 1), // Left, Right
            (1, -1),
            (1, 0),
            (1, 1), // Bottom-left, Bottom, Bottom-right
        ];

        let mut sum_gear_ratios = 0i64;

        // Process each asterisk
        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == '*' {
                    let mut adjacent_numbers = vec![];

                    for &(dr, dc) in &directions {
                        let adj_row = (r as isize + dr) as usize;
                        let adj_col = (c as isize + dc) as usize;

                        if adj_row < rows && adj_col < cols {
                            if let Some(number) = parse_number_at(&matrix, adj_row, adj_col) {
                                if !adjacent_numbers.contains(&number) {
                                    // Check for duplicates
                                    adjacent_numbers.push(number);
                                }
                            }
                        }
                    }

                    if adjacent_numbers.len() == 2 {
                        sum_gear_ratios += adjacent_numbers[0] * adjacent_numbers[1]
                    }
                }
            }
        }

        sum_gear_ratios
    }

    pub fn part_2(&self, _input: &str) -> String {
        let matrix = Self::parse_input(_input);
        let sum = Self::find_gear_ratios(&matrix);
        sum.to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day03 for AoC 2023.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2023_day03_part_1() {
        use crate::solutions::aoc2023::Day03;
        let day03 = Day03 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#;
        assert_eq!(day03.part_1(input), "4361"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day03 for AoC 2023.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2023_day03_part_2() {
        use crate::solutions::aoc2023::Day03;
        let day03 = Day03 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#;
        assert_eq!(day03.part_2(input), "467835"); // Asserts if the function output matches the expected result.
    }
}
