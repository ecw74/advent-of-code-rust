use super::day07::Day07;

impl Day07 {
    /// Parses the input into a vector of tuples containing the target value (u64) and a vector of numbers (u16).
    fn parse_test_values(input: &str) -> Result<Vec<(u64, Vec<u16>)>, String> {
        input
            .lines()
            .map(str::trim) // Remove leading and trailing whitespace from each line.
            .filter(|line| !line.is_empty()) // Skip empty lines.
            .map(|line| {
                let mut parts = line.split(':'); // Split each line into the target and number list.

                // Parse the target value (to the left of the colon).
                let key = parts
                    .next()?
                    .trim()
                    .parse::<u64>() // Parse as u64 since target values can be large.
                    .ok()?;

                // Parse the list of numbers (to the right of the colon).
                let values = parts
                    .next()?
                    .trim()
                    .split_whitespace()
                    .map(|v| v.parse::<u16>().ok()) // Parse as u16; these values are smaller.
                    .collect::<Option<Vec<u16>>>()?;

                Some((key, values)) // Return the tuple if successful.
            })
            .collect::<Option<Vec<(u64, Vec<u16>)>>>() // Collect all parsed lines into a vector.
            .ok_or_else(|| "Failed to parse test values".to_string()) // Return an error if parsing fails.
    }

    /// Calculates the total calibration result by evaluating all possible operator combinations.
    fn calculate_total_calibration(
        test_values: Vec<(u64, Vec<u16>)>, // Parsed test values as input.
        base: u32, // The number of operators (e.g., 2 for Part 1, 3 for Part 2).
    ) -> String {
        let mut total_calibration_result = 0;

        // Iterate over each (target value, numbers list) tuple.
        for (result, numbers) in test_values {
            // Calculate the number of operator combinations (base^(n-1)).
            let num_combinations = base.pow(u32::try_from(numbers.len() - 1).unwrap());
            for mut i in 0..num_combinations {
                let mut numbers_iter = numbers.iter(); // Create an iterator over the numbers.
                let mut test_result = *numbers_iter.next().unwrap() as u64; // Start with the first number.

                // Apply operators between numbers.
                for n in numbers_iter {
                    match i % base {
                        0 => test_result += *n as u64,
                        1 => test_result *= *n as u64,
                        3 => {
                            test_result *= 10_u64.pow(n.ilog10() + 1);
                            test_result += *n as u64;
                        }
                        _ => {}
                    }
                    i /= base; // Move to the next operator combination.
                }

                // Check if the calculated result matches the target.
                if test_result == result {
                    total_calibration_result += result; // Add the target value to the total if it matches.
                    break; // No need to try further combinations for this target.
                }
            }
        }
        total_calibration_result.to_string() // Return the total as a string.
    }

    /// Part 1: Solve using only addition (+) and multiplication (*).
    pub fn part_1(&self, input: &str) -> String {
        let test_values = Self::parse_test_values(input).unwrap(); // Parse the input.
        Self::calculate_total_calibration(test_values, 2)
    }

    /// Part 2: Solve using addition (+), multiplication (*), and concatenation (||).
    pub fn part_2(&self, input: &str) -> String {
        let test_values = Self::parse_test_values(input).unwrap(); // Parse the input.
        Self::calculate_total_calibration(test_values, 3)
    }
}

mod test {
    #[test]
    /// Test for Part 1 of Day 7. Validates calculation of target sums for addition and multiplication operators.
    fn test_aoc2024_day07_part_1() {
        use crate::solutions::aoc2024::Day07;
        let day07 = Day07 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "#;
        assert_eq!(day07.part_1(input), "3749"); // Check against the expected result for Part 1.
    }

    #[test]
    /// Test for Part 2 of Day 7. Validates calculation with the concatenation operator.
    fn test_aoc2024_day07_part_2() {
        use crate::solutions::aoc2024::Day07;
        let day07 = Day07 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "#;
        assert_eq!(day07.part_2(input), "11387"); // Check against the expected result for Part 2.
    }
}
