use super::day04::Day04; // Import Day04 from its module.

impl Day04 {
    /// Parses the input into a vector of match counts for each card.
    /// Each match count represents the number of matching numbers between the winning set and the player's set.
    fn parse_cards(input: &str) -> Vec<usize> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines.
            .map(|line| {
                let parts: Vec<&str> = line.split('|').collect(); // Split the line into two parts using '|' as the delimiter.
                if parts.len() == 2 {
                    let left_numbers = parts[0]
                        .split(':')
                        .last()
                        .unwrap_or(parts[0]) // Handle cases where there's a ':' prefix.
                        .split_whitespace() // Split the winning numbers by whitespace.
                        .filter_map(|num| num.parse::<u32>().ok()); // Parse numbers into `u32`.

                    let right_numbers = parts[1]
                        .split_whitespace() // Split the player's numbers by whitespace.
                        .filter_map(|num| num.parse::<u32>().ok()); // Parse numbers into `u32`.

                    // Count how many numbers in the player's set match the winning numbers.
                    left_numbers.filter(|&n| right_numbers.clone().any(|m| m == n)).count()
                } else {
                    panic!("Invalid line format: {}", line); // Ensure proper format for each line.
                }
            })
            .collect() // Collect the match counts into a vector.
    }

    /// Solves Part 1: Calculate the total points for the scratchcards.
    pub fn part_1(&self, input: &str) -> String {
        let matches = Self::parse_cards(input); // Parse the input into match counts.
        let sum: u32 = matches
            .iter()
            .map(|&m| {
                if m > 0 {
                    1u32 << (m - 1) // Calculate points as 2^(matches-1).
                } else {
                    0 // If no matches, the card is worth 0 points.
                }
            })
            .sum(); // Sum all card points.
        sum.to_string() // Convert the total points to a string and return.
    }

    /// Solves Part 2: Calculate the total number of scratchcards, including all copies.
    pub fn part_2(&self, input: &str) -> String {
        let matches = Self::parse_cards(input); // Parse the input into match counts.
        let mut copies = vec![1u32; matches.len()]; // Start with 1 copy per original card.

        for i in 0..matches.len() {
            if copies[i] > 0 {
                let m = matches[i]; // Get the number of matches for the current card.

                // Distribute copies to the next `m` cards.
                for j in i + 1..std::cmp::min(i + 1 + m, matches.len()) {
                    copies[j] += copies[i]; // Add copies from the current card to subsequent cards.
                }
            }
        }

        copies.iter().sum::<u32>().to_string() // Sum all copies and return the total as a string.
    }
}

mod test {
    /// Tests for Part 1 and Part 2 of Day 4.

    #[test]
    /// Test for Part 1: Validates the point calculation logic for a sample input.
    fn test_aoc2023_day04_part_1() {
        use crate::solutions::aoc2023::Day04; // Import Day04 from its namespace.
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(day04.part_1(input), "13"); // Expected output: 13 points.
    }

    #[test]
    /// Test for Part 2: Validates the cascading logic for a sample input.
    fn test_aoc2023_day04_part_2() {
        use crate::solutions::aoc2023::Day04; // Import Day04 from its namespace.
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#;
        assert_eq!(day04.part_2(input), "30"); // Expected output: 30 total cards.
    }
}
