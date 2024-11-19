use super::day02::Day02;

impl Day02 {
    /// Part 1 of the challenge.
    /// Processes input data to compute a specific result based on game rules.
    /// Each line of the input represents a game round.
    /// The function calculates points based on specific game outcomes.
    pub fn part_1(&self, input: &str) -> String {
        let sum = input
            .split("\n") // Split the input into lines.
            .filter(|s| !s.is_empty()) // Filter out empty lines.
            .filter_map(|game| {
                let round: Vec<&str> = game.split_whitespace().collect::<Vec<&str>>();
                if round.len() < 2 {
                    return None; // Skip rounds with insufficient data.
                }
                let points = match round[1] {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    _ => 0,
                }; // Assign points.
                match (round[0], round[1]) {
                    // Specific combinations yield additional points.
                    ("A", "Z") | ("C", "Y") | ("B", "X") => Some(points + 0),
                    ("C", "X") | ("B", "Z") | ("A", "Y") => Some(points + 6),
                    _ => Some(points + 3),
                }
            })
            .sum::<i32>();
        sum.to_string() // Convert the final result to a String
    }

    /// Part 2 of the challenge.
    /// Similar to part_1 but with a different point system.
    /// Points are calculated based on different outcomes and conditions.
    pub fn part_2(&self, input: &str) -> String {
        let sum = input
            .split("\n") // Split the input into lines.
            .filter(|s| !s.is_empty()) // Filter out empty lines.
            .filter_map(|game| {
                let round: Vec<&str> = game.split_whitespace().collect::<Vec<&str>>();
                if round.len() < 2 {
                    return None; // Skip rounds with insufficient data.
                }
                match round[1] {
                    // Points allocation based on the second element of the round.
                    // "X" indicates a loss, "Y" a draw, and "Z" a win.
                    "X" => match round[0] {
                        "A" => Some(3),
                        "B" => Some(1),
                        "C" => Some(2),
                        _ => Some(0),
                    },
                    "Y" => match round[0] {
                        "A" => Some(3 + 1),
                        "B" => Some(3 + 2),
                        "C" => Some(3 + 3),
                        _ => Some(0),
                    },
                    "Z" => match round[0] {
                        "A" => Some(6 + 2),
                        "B" => Some(6 + 3),
                        "C" => Some(6 + 1),
                        _ => Some(0),
                    },
                    _ => Some(0),
                }
            })
            .sum::<i32>();
        sum.to_string() // Convert the final result to a String
    }
}

mod test {
    #[test]
    /// Tests for part 1 of Day02 of AoC 2022.
    /// Verifies that the function computes the correct result for a given input.
    fn test_aoc2022_day02_part_1() {
        use crate::solutions::aoc2022::Day02;
        let day02 = Day02 {
            day: 2,
            year: 22,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(day02.part_1(input), "15"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Tests for part 2 of Day02 of AoC 2022.
    /// Checks if the function correctly calculates the result for another point system.
    fn test_aoc2022_day02_part_2() {
        use crate::solutions::aoc2022::Day02;
        let day02 = Day02 {
            day: 2,
            year: 22,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
A Y
B X
C Z
"#;
        assert_eq!(day02.part_2(input), "12"); // Asserts if the function output matches the expected result.
    }
}
