use super::day02::Day02;

impl Day02 {
    /// Parses a string into a Vec<Vec<i32>> by splitting it line by line
    /// and converting space-separated numbers into integers.
    fn parse_to_vectors(input: &str) -> Vec<Vec<i32>> {
        input
            .lines() // Split the input into lines.
            .filter(|s| !s.trim().is_empty()) // Remove any empty lines.
            .map(|line| {
                line.split_whitespace() // Split the line into words based on spaces.
                    .filter_map(|num| num.parse::<i32>().ok()) // Attempt to parse each word as an i32.
                    .collect() // Collect the parsed integers into a Vec<i32>.
            })
            .collect() // Collect all the parsed Vec<i32> into a Vec<Vec<i32>>.
    }

    /// Checks if a report (a vector of levels) is "safe" according to the rules:
    /// - Levels must either be all increasing or all decreasing.
    /// - Differences between adjacent levels must be between 1 and 3 (inclusive).
    fn is_safe_report(report: &[i32]) -> bool {
        // Check if all adjacent differences are between 1 and 3 and are consistently increasing.
        report.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])))
            // Check if all adjacent differences are between 1 and 3 and are consistently decreasing.
            || report.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])))
    }

    /// Solves Part 1 of the puzzle.
    /// Counts the number of reports that are "safe" without any modifications.
    pub fn part_1(&self, _input: &str) -> String {
        let level_vectors = Self::parse_to_vectors(_input); // Parse the input into a Vec<Vec<i32>>.

        // Count the number of safe reports and convert the count to a String.
        level_vectors
            .iter() // Iterate over each report.
            .filter(|report| Self::is_safe_report(report)) // Keep only the reports that are safe.
            .count() // Count the number of safe reports.
            .to_string() // Convert the count to a String for the final output.
    }

    /// Solves Part 2 of the puzzle.
    /// Counts the number of reports that are "safe" with the Problem Dampener:
    /// - Allows removing one "bad" level to make the report safe.
    pub fn part_2(&self, _input: &str) -> String {
        let level_vectors = Self::parse_to_vectors(_input); // Parse the input into a Vec<Vec<i32>>.

        // Calculate the total number of safe reports, considering the Problem Dampener.
        level_vectors
            .iter() // Iterate over each report.
            .map(|report| {
                if Self::is_safe_report(report) {
                    1 // If the report is already safe, count it as 1.
                } else {
                    // Otherwise, check if removing any single level makes the report safe (brute force).
                    (0..report.len()).any(|i| {
                        // Create a new report by removing the level at index `i`.
                        let sub_report: Vec<_> = report
                            .iter()
                            .copied() // Copy each value to avoid references.
                            .enumerate()
                            .filter(|&(j, _)| j != i) // Skip the level at index `i`.
                            .map(|(_, value)| value) // Extract the level values.
                            .collect(); // Collect the remaining levels into a Vec<i32>.
                        Self::is_safe_report(&sub_report)
                    }) as u32 // Convert the boolean to 1 or 0.
                }
            })
            .sum::<u32>() // Sum up the counts of all safe reports.
            .to_string() // Convert the sum to a String for the final output.
    }
}

mod test {
    #[test]
    /// Test for Part 1 of Day02.
    /// Ensures correct calculation of the number of safe reports without modifications.
    fn test_aoc2024_day02_part_1() {
        use crate::solutions::aoc2024::Day02;
        let day02 = Day02 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#;

        // Expected result: 2 safe reports in this input.
        assert_eq!(day02.part_1(input), "2");
    }

    #[test]
    /// Test for Part 2 of Day02.
    /// Verifies correct calculation of the number of safe reports with the Problem Dampener.
    fn test_aoc2024_day02_part_2() {
        use crate::solutions::aoc2024::Day02;
        let day02 = Day02 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#;

        // Expected result: 4 safe reports with the Problem Dampener.
        assert_eq!(day02.part_2(input), "4");
    }
}
