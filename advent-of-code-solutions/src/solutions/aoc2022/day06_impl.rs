use super::day06::Day06;

impl Day06 {
    /// Finds the position of the first "marker" in the datastream.
    ///
    /// A "marker" is defined as a sequence of `length` consecutive characters that are all unique.
    fn find_start_of_message_marker(message: &str, length: usize) -> u32 {
        // Convert the input string into a vector of characters for easy slicing.
        let chars: Vec<char> = message.chars().collect();

        // Initialize a variable to store the result (default to 0 if no marker is found).
        let mut result = 0u32;

        // Iterate over the message, starting from `length - 1` to account for the window size.
        for i in (length - 1)..chars.len() {
            // Extract a slice of `length` consecutive characters.
            let slice = &chars[i - (length - 1)..=i];

            // Use a HashSet to check for uniqueness in the current slice.
            let unique: std::collections::HashSet<&char> = slice.iter().collect();

            // If the number of unique characters equals the required `length`, we've found the marker.
            if unique.len() == length {
                // The position in the stream is 1-based, so add 1 to `i`.
                result = (i + 1) as u32;
                break; // Stop searching after finding the first marker.
            }
        }

        // Return the result.
        result
    }

    /// Solves Part 1 of the puzzle.
    ///
    /// Finds the position of the first "start-of-packet marker" in the datastream,
    /// which is defined as a sequence of 4 unique characters.
    pub fn part_1(&self, _input: &str) -> String {
        Self::find_start_of_message_marker(_input, 4).to_string()
    }

    /// Solves Part 2 of the puzzle.
    ///
    /// Finds the position of the first "start-of-message marker" in the datastream,
    /// which is defined as a sequence of 14 unique characters.
    pub fn part_2(&self, _input: &str) -> String {
        Self::find_start_of_message_marker(_input, 14).to_string()
    }
}

mod test {
    #[test]
    /// Test for Part 1 of Day 06.
    ///
    /// Verifies that the `part_1` method correctly finds the position of the first
    /// start-of-packet marker (4 unique characters).
    fn test_aoc2022_day06_part_1() {
        use crate::solutions::aoc2022::Day06;

        // Create a Day06 instance.
        let day06 = Day06 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };

        // Test input string.
        let input = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;

        // Assert that the result matches the expected output (5 in this case).
        assert_eq!(day06.part_1(input), "5");
    }

    #[test]
    /// Test for Part 2 of Day 06.
    ///
    /// Verifies that the `part_2` method correctly finds the position of the first
    /// start-of-message marker (14 unique characters).
    fn test_aoc2022_day06_part_2() {
        use crate::solutions::aoc2022::Day06;

        // Create a Day06 instance.
        let day06 = Day06 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };

        // Test input string.
        let input = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;

        // Assert that the result matches the expected output (23 in this case).
        assert_eq!(day06.part_2(input), "23");
    }
}
