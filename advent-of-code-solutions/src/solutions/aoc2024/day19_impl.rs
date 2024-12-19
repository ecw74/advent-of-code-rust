use super::day19::Day19;
use hashbrown::HashSet;

impl Day19 {
    fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
        let mut lines = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty()); // Ignore empty lines

        // Parse the first line for towels
        let towels_line = lines
            .next()
            .expect("Input must contain at least one line for towels.");
        let towels: HashSet<String> = towels_line
            .split(',')
            .map(|t| t.trim().to_string())
            .collect();

        // Parse the rest as patterns
        let patterns: Vec<String> = lines.map(|line| line.to_string()).collect();

        (towels, patterns)
    }

    fn can_form_pattern(towels: &HashSet<String>, pattern: &str) -> bool {
        let n = pattern.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true; // Base case: Empty pattern can always be formed

        for i in 1..=n {
            for j in 0..i {
                if dp[j] && towels.contains(&pattern[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }

    pub fn part_1(&self, input: &str) -> String {
        let (towels, patterns) = Self::parse_input(input);

        let mut count = 0u16;
        for pattern in patterns {
            if Self::can_form_pattern(&towels, &pattern) {
                count += 1;
            }
        }

        count.to_string()
    }

    fn count_combinations(towels: &HashSet<String>, pattern: &str) -> u64 {
        let n = pattern.len();
        let mut dp = vec![0u64; n + 1];
        dp[0] = 1; // Base case: One way to form the empty pattern

        for i in 1..=n {
            for j in 0..i {
                if towels.contains(&pattern[j..i]) {
                    dp[i] += dp[j];
                }
            }
        }

        dp[n]
    }

    pub fn part_2(&self, input: &str) -> String {
        let (towels, patterns) = Self::parse_input(input);
        // Calculate total combinations
        let mut total_combinations = 0u64;
        for pattern in patterns {
            total_combinations += Self::count_combinations(&towels, &pattern);
        }
        println!("Total combinations: {}", total_combinations);
        total_combinations.to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day19 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
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
        assert_eq!(day19.part_1(input), "6"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day19 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
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
        assert_eq!(day19.part_2(input), "16"); // Asserts if the function output matches the expected result.
    }
}
