use super::day11::Day11;
use std::collections::HashMap;

impl Day11 {
    /// Parses a string input into a vector of unsigned 64-bit integers (`u64`).
    /// - Splits the input string by whitespace.
    /// - Attempts to parse each segment into a number, ignoring invalid entries.
    fn parse_numbers(input: &str) -> Vec<u64> {
        input
            .split_whitespace() // Split the string by any whitespace.
            .filter_map(|s| s.parse::<u64>().ok()) // Parse each segment as `u64`, skipping failures.
            .collect() // Collect the valid numbers into a vector.
    }

    /// Simulates the evolution of stones for a given number of blinks.
    /// - `stones`: Initial arrangement of stones.
    /// - `blinks`: Number of times the stones change.
    /// Returns the total number of stones after the specified number of blinks.
    fn count_stones_after_blinks(stones: Vec<u64>, blinks: usize) -> usize {
        let mut memo = HashMap::new(); // Memoization cache for recursive results.

        // Recursive helper function to calculate the number of stones for a single stone.
        fn stones_recursive(
            c: u64,                                  // Current stone's value.
            n: usize,                                // Remaining number of blinks.
            memo: &mut HashMap<(u64, usize), usize>, // Memoization cache.
        ) -> usize {
            if n == 0 {
                return 1; // Base case: No blinks left, one stone remains.
            }

            // Return the cached result if already computed.
            if let Some(&result) = memo.get(&(c, n)) {
                return result;
            }

            // Compute the result based on the puzzle rules.
            let result = if c == 0 {
                // Rule 1: If the stone is 0, it becomes 1.
                stones_recursive(1, n - 1, memo)
            } else {
                let c_str = c.to_string(); // Convert the number to a string to analyze its digits.
                if c_str.len() % 2 != 0 {
                    // Rule 3: If the number has an odd number of digits, multiply by 2024.
                    stones_recursive(c * 2024, n - 1, memo)
                } else {
                    // Rule 2: If the number has an even number of digits, split into two stones.
                    let mid = c_str.len() / 2; // Find the middle index of the digits.
                    let left = c_str[..mid].parse::<u64>().unwrap(); // Left half as a new stone.
                    let right = c_str[mid..].parse::<u64>().unwrap(); // Right half as a new stone.
                    stones_recursive(left, n - 1, memo) + stones_recursive(right, n - 1, memo)
                }
            };

            // Cache the computed result.
            memo.insert((c, n), result);
            result
        }

        // Sum the number of stones for all initial stones after the specified number of blinks.
        stones
            .into_iter()
            .map(|c| stones_recursive(c, blinks, &mut memo)) // Process each stone recursively.
            .sum() // Add up the results for all stones.
    }

    /// Solves part 1 of the puzzle.
    /// - Parses the input, simulates 25 blinks, and returns the result as a string.
    pub fn part_1(&self, input: &str) -> String {
        let stones = Self::parse_numbers(input); // Parse the input into numbers.
        Self::count_stones_after_blinks(stones, 25).to_string() // Simulate and convert result to string.
    }

    /// Solves part 2 of the puzzle.
    /// - Parses the input, simulates 75 blinks, and returns the result as a string.
    pub fn part_2(&self, input: &str) -> String {
        let stones = Self::parse_numbers(input); // Parse the input into numbers.
        Self::count_stones_after_blinks(stones, 75).to_string() // Simulate and convert result to string.
    }
}

// Test module for verifying the solution.
mod test {
    #[test]
    /// Test for part 1 of Day 11.
    /// Ensures the correct number of stones is calculated after 25 blinks.
    fn test_aoc2024_day11_part_1() {
        use crate::solutions::aoc2024::Day11;
        let day11 = Day11 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
125 17
        "#;
        // Expected result after 25 blinks is 55312.
        assert_eq!(day11.part_1(input), "55312");
    }

    #[test]
    /// Test for part 2 of Day 11.
    /// Verifies the correct number of stones after 75 blinks.
    fn test_aoc2024_day11_part_2() {
        use crate::solutions::aoc2024::Day11;
        let day11 = Day11 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
125 17
        "#;
        // Expected result after 75 blinks is 65601038650482.
        assert_eq!(day11.part_2(input), "65601038650482");
    }
}
