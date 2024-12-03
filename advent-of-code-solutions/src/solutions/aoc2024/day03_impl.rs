use super::day03::Day03;
use lazy_static::lazy_static;

use regex::Regex;

// Use compile-time regex to reduce memory usage
lazy_static! {
    static ref AOC_2024_03_OUTER_REGEX: Regex = Regex::new(r"(mul|do|don't)\(([^()]*?)\)").unwrap();
    static ref AOC_2024_03_INNER_REGEX: Regex = Regex::new(r"^(\d+),\s*(\d+)$").unwrap();
}

impl Day03 {
    /// Parses the input string to extract valid instructions and their arguments.
    /// Returns a vector of tuples where the first element is the instruction name
    /// (e.g., "mul", "do", "don't"), and the second element is a vector of arguments.
    fn parse_instructions(input: &str) -> Vec<(String, Vec<u32>)> {
        let mut results = Vec::new();

        // Iterate over all matches found by the main regex
        for caps in AOC_2024_03_OUTER_REGEX.captures_iter(input) {
            let func_name = &caps[1]; // Capture the function name (e.g., "mul")
            let args = &caps[2]; // Capture the arguments inside the parentheses

            match func_name {
                "mul" => {
                    // Use a secondary regex to parse arguments for "mul"
                    if let Some(arg_caps) = AOC_2024_03_INNER_REGEX.captures(args) {
                        let arg1: u32 = arg_caps[1].parse().unwrap(); // Parse the first argument
                        let arg2: u32 = arg_caps[2].parse().unwrap(); // Parse the second argument
                        results.push((func_name.to_string(), vec![arg1, arg2]));
                        // Store the parsed instruction
                    }
                }
                "do" | "don't" => {
                    // For "do" and "don't", simply add the instruction with no arguments
                    results.push((func_name.to_string(), vec![]));
                }
                _ => {}
            }
        }

        results
    }

    /// Part 1: Calculates the sum of all valid "mul(X,Y)" results in the input.
    pub fn part_1(&self, input: &str) -> String {
        // Parse the input to extract instructions
        let pairs = Self::parse_instructions(input);

        // Calculate the sum of all "mul" results
        let sum: u32 = pairs
            .iter()
            .filter_map(|(inst, args)| {
                if inst == "mul" {
                    Some(args[0] * args[1])
                } else {
                    None
                }
            })
            .sum();

        sum.to_string() // Return the result as a string
    }

    /// Part 2: Handles enabling/disabling of "mul" based on "do()" and "don't()".
    pub fn part_2(&self, input: &str) -> String {
        // Parse the input to extract instructions
        let pairs = Self::parse_instructions(input);

        let mut mul_enabled = true; // "mul" is initially enabled
        let mut result = 0; // Initialize the result sum

        for (inst, args) in pairs {
            match inst.as_str() {
                "do" => mul_enabled = true,     // Enable "mul" instructions
                "don't" => mul_enabled = false, // Disable "mul" instructions
                "mul" if mul_enabled => {
                    // Process "mul" only if enabled
                    result += args[0] * args[1];
                }
                _ => {}
            }
        }

        result.to_string() // Return the final result as a string
    }
}

// Test module for Day03
mod test {
    #[test]
    /// Test for Part 1 of Day03 to ensure correct handling of all valid "mul" instructions.
    fn test_aoc2024_day03_part_1() {
        use crate::solutions::aoc2024::Day03; // Import the Day03 struct
        let day03 = Day03 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!(day03.part_1(input), "161"); // Expected result for Part 1
    }

    #[test]
    /// Test for Part 2 of Day03 to verify correct handling of "do()" and "don't()" instructions.
    fn test_aoc2024_day03_part_2() {
        use crate::solutions::aoc2024::Day03; // Import the Day03 struct
        let day03 = Day03 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!(day03.part_2(input), "48"); // Expected result for Part 2
    }
}
