use super::day19::Day19;
use hashbrown::HashSet; // Efficient hash-based set for storing towel patterns.

impl Day19 {
    /// Parses the input into two components:
    /// - A set of available towel patterns.
    /// - A list of desired designs to form.
    ///
    /// # Arguments
    /// - `input`: A string containing the input data.
    ///
    /// # Returns
    /// A tuple `(HashSet<String>, Vec<String>)` where:
    /// - The first element is a set of towel patterns.
    /// - The second element is a vector of desired designs.
    fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
        // Split the input into lines and ignore empty ones.
        let mut lines = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty());

        // Parse the first line containing towel patterns, separated by commas.
        let towels_line = lines
            .next()
            .expect("Input must contain at least one line for towels.");
        let towels: HashSet<String> = towels_line
            .split(',')
            .map(|t| t.trim().to_string()) // Clean up and convert to String.
            .collect();

        // Parse the remaining lines as desired designs.
        let patterns: Vec<String> = lines.map(|line| line.to_string()).collect();

        (towels, patterns)
    }

    /// Checks if a given pattern can be formed using the available towel patterns.
    ///
    /// # Arguments
    /// - `towels`: A set of available towel patterns.
    /// - `pattern`: The desired design to check.
    ///
    /// # Returns
    /// A boolean indicating whether the pattern can be formed.
    fn can_form_pattern(towels: &HashSet<String>, pattern: &str) -> bool {
        // Dynamic programming array to track feasible substrings.
        let mut dp = vec![false; pattern.len() + 1];
        dp[0] = true; // Base case: An empty pattern is always feasible.

        for i in 1..=pattern.len() {
            for j in 0..i {
                // Check if the substring pattern[j..i] matches any towel pattern.
                if dp[j] && towels.contains(&pattern[j..i]) {
                    dp[i] = true;
                    break; // Once a match is found, no need to check further.
                }
            }
        }

        dp[pattern.len()] // The last entry indicates if the full pattern is feasible.
    }

    /// Solves Part 1: Counts how many desired designs can be formed.
    ///
    /// # Arguments
    /// - `input`: The puzzle input as a string.
    ///
    /// # Returns
    /// A string representing the count of feasible designs.
    pub fn part_1(&self, input: &str) -> String {
        let (towels, patterns) = Self::parse_input(input);
        patterns
            .iter()
            .filter(|pattern| Self::can_form_pattern(&towels, pattern))
            .count()
            .to_string()
    }

    /// Counts the number of ways a given pattern can be formed using the available towel patterns.
    ///
    /// # Arguments
    /// - `towels`: A set of available towel patterns.
    /// - `pattern`: The desired design to check.
    ///
    /// # Returns
    /// The number of distinct ways the pattern can be formed.
    fn count_combinations(towels: &HashSet<String>, pattern: &str) -> u64 {
        // Dynamic programming array to track the number of ways to form substrings.
        let mut dp = vec![0u64; pattern.len() + 1];
        dp[0] = 1; // Base case: One way to form an empty pattern.

        for i in 1..=pattern.len() {
            for j in 0..i {
                // If the substring pattern[j..i] matches a towel pattern,
                // add the number of ways to form the prefix ending at j.
                if towels.contains(&pattern[j..i]) {
                    dp[i] += dp[j];
                }
            }
        }

        dp[pattern.len()] // The last entry contains the total combinations.
    }

    /// Solves Part 2: Calculates the sum of all possible arrangements for feasible designs.
    ///
    /// # Arguments
    /// - `input`: The puzzle input as a string.
    ///
    /// # Returns
    /// A string representing the total number of arrangements.
    pub fn part_2(&self, input: &str) -> String {
        let (towels, patterns) = Self::parse_input(input);
        patterns
            .iter()
            .map(|pattern| Self::count_combinations(&towels, pattern))
            .sum::<u64>()
            .to_string()
    }
}

mod test {
    #[test]
    /// Test for Part 1: Checks if the number of feasible designs is correct.
    fn test_aoc2024_day19_part_1() {
        use crate::solutions::aoc2024::Day19;
        let day19 = Day19 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
        "#;
        assert_eq!(day19.part_1(input), "6"); // Verifies Part 1 output.
    }

    #[test]
    /// Test for Part 2: Checks if the total arrangements are calculated correctly.
    fn test_aoc2024_day19_part_2() {
        use crate::solutions::aoc2024::Day19;
        let day19 = Day19 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
        "#;
        assert_eq!(day19.part_2(input), "16"); // Verifies Part 2 output.
    }
}
