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
    /// - `initial_stones`: The initial sequence of numbers on the stones.
    /// - `blinks`: The number of transformation iterations to perform.
    /// Returns the total number of stones after all blinks.
    fn simulate_blinks(initial_stones: &Vec<u64>, blinks: usize) -> u64 {
        let mut stones = Vec::with_capacity(5000); // Tracks transformations between stone indices.
        let mut indices = HashMap::with_capacity(5000); // Maps stone numbers to indices.
        let mut todo = Vec::new(); // Holds new stone numbers to process in the current blink.
        let mut current = Vec::new(); // Tracks the count of stones by index.

        // Initialize stones from the input.
        for &number in initial_stones {
            if let Some(&index) = indices.get(&number) {
                current[index] += 1; // Increment count if the stone number is already tracked.
            } else {
                indices.insert(number, indices.len()); // Assign a new index to the stone number.
                todo.push(number); // Queue this stone number for processing.
                current.push(1); // Initialize the count for this new stone.
            }
        }

        for _ in 0..blinks {
            let numbers = todo; // Take the stones queued for this blink.
            todo = Vec::with_capacity(200); // Prepare a new queue for the next blink.

            // A closure to map a stone number to its index or queue it for future processing.
            let mut index_of = |number| {
                let size = indices.len();
                *indices.entry(number).or_insert_with(|| {
                    todo.push(number); // Queue the number for future processing.
                    size // Assign a new index.
                })
            };

            // Transform each stone number according to the rules.
            for number in numbers {
                let (first, second) = if number == 0 {
                    (index_of(1), usize::MAX) // Rule 1: Replace 0 with 1.
                } else {
                    let digits = number.ilog10() + 1; // Count digits in the number.
                    if digits % 2 == 0 {
                        // Rule 2: Split even-digit numbers.
                        let power = 10_u64.pow(digits / 2);
                        (index_of(number / power), index_of(number % power))
                    } else {
                        // Rule 3: Multiply other numbers by 2024.
                        (index_of(number * 2024), usize::MAX)
                    }
                };

                stones.push((first, second)); // Record transformations for this stone.
            }

            // Update stone counts based on transformations.
            let mut next = vec![0; indices.len()];
            for (&(first, second), amount) in stones.iter().zip(current) {
                next[first] += amount; // Add the count to the first resulting stone.
                if second != usize::MAX {
                    next[second] += amount; // Add the count to the second resulting stone, if any.
                }
            }
            current = next; // Move to the next iteration.
        }

        current.iter().sum() // Sum all stone counts to get the total.
    }

    /// Solves part 1 of the puzzle by simulating 25 blinks.
    /// Parses the input, simulates the transformations, and returns the result.
    pub fn part_1(&self, input: &str) -> String {
        let stones = Self::parse_numbers(input); // Parse the initial stones.
        Self::simulate_blinks(&stones, 25).to_string() // Simulate and convert the result to a string.
    }

    /// Solves part 2 of the puzzle by simulating 75 blinks.
    /// Parses the input, simulates the transformations, and returns the result.
    pub fn part_2(&self, input: &str) -> String {
        let stones = Self::parse_numbers(input); // Parse the initial stones.
        Self::simulate_blinks(&stones, 75).to_string() // Simulate and convert the result to a string.
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
