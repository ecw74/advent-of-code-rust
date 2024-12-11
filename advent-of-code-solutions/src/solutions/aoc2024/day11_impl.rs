use super::day11::Day11;
use std::collections::HashMap;

impl Day11 {
    /// Parses a string input into a vector of unsigned 64-bit integers (`u64`).
    /// This function handles:
    /// - Splitting the input string by whitespace.
    /// - Parsing each segment into a `u64`.
    /// - Ignoring any invalid entries (e.g., non-numeric strings).
    fn parse_numbers(input: &str) -> Vec<u64> {
        input
            .split_whitespace() // Split the string into segments based on whitespace.
            .filter_map(|s| s.parse::<u64>().ok()) // Try parsing each segment into a `u64`. Skip errors.
            .collect() // Collect the successfully parsed numbers into a vector.
    }

    /// Simulates the transformation of stones for a specified number of iterations (blinks).
    /// - `initial_stones` is the initial sequence of numbers on the stones.
    /// - `blinks` is the number of transformation iterations to perform.
    /// Returns the total number of stones after all blinks.
    fn simulate_blinks(initial_stones: Vec<u64>, blinks: usize) -> usize {
        // HashMap to track the count of each unique stone number.
        let mut stone_counts: HashMap<u64, usize> = HashMap::new();

        // Populate the initial counts for the given stones.
        for &stone in &initial_stones {
            *stone_counts.entry(stone).or_insert(0) += 1;
        }

        // Perform transformations for the specified number of blinks.
        for _ in 0..blinks {
            // Temporary HashMap to hold the next generation of stones.
            let mut next_stone_counts: HashMap<u64, usize> = HashMap::new();

            // Process each unique stone number and its count.
            for (&stone, &count) in &stone_counts {
                if stone == 0 {
                    // Rule 1: Replace stones engraved with `0` with stones engraved `1`.
                    *next_stone_counts.entry(1).or_insert(0) += count;
                } else if stone.to_string().len() % 2 == 0 {
                    // Rule 2: Split stones with an even number of digits.
                    let digits = stone.to_string(); // Convert the number to a string to find its length.
                    let mid = digits.len() / 2; // Find the midpoint for splitting.
                    let left = digits[..mid].parse::<u64>().unwrap(); // Left part of the split.
                    let right = digits[mid..].parse::<u64>().unwrap(); // Right part of the split.
                    *next_stone_counts.entry(left).or_insert(0) += count; // Add the left half.
                    *next_stone_counts.entry(right).or_insert(0) += count; // Add the right half.
                } else {
                    // Rule 3: Replace stones with their value multiplied by 2024.
                    let new_stone = stone * 2024;
                    *next_stone_counts.entry(new_stone).or_insert(0) += count;
                }
            }

            // Move to the next generation of stones.
            stone_counts = next_stone_counts;
        }

        // Sum the total count of all stones.
        stone_counts.values().sum()
    }

    /// Solves part 1 of the puzzle by simulating 25 blinks.
    /// Parses the input, simulates the transformations, and returns the result.
    pub fn part_1(&self, input: &str) -> String {
        let stones = Self::parse_numbers(input); // Parse the initial stones.
        Self::simulate_blinks(stones, 25).to_string() // Simulate and convert the result to a string.
    }

    /// Solves part 2 of the puzzle by simulating 75 blinks.
    /// Parses the input, simulates the transformations, and returns the result.
    pub fn part_2(&self, input: &str) -> String {
        let stones = Self::parse_numbers(input); // Parse the initial stones.
        Self::simulate_blinks(stones, 75).to_string() // Simulate and convert the result to a string.
    }
}

// Test module to verify correctness of the solution for both parts.
mod test {
    #[test]
    /// Test for part 1 of Day 11.
    /// Checks if the correct number of stones is computed after 25 blinks.
    fn test_aoc2024_day11_part_1() {
        use crate::solutions::aoc2024::Day11; // Import the solution module.
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
    /// Verifies the number of stones after 75 blinks.
    fn test_aoc2024_day11_part_2() {
        use crate::solutions::aoc2024::Day11; // Import the solution module.
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
