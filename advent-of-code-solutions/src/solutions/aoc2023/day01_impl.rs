use super::day01::Day01;

impl Day01 {
    // Part 1 of the challenge: Summing up the first and last digits of each line.
    pub fn part_1(&self, input: &str) -> i64 {
        input
            .split("\n") // Splitting the input string into lines.
            .filter(|s| !s.trim().is_empty()) // Filtering out empty lines.
            .filter_map(|line| {
                // Processing each line.
                let digits: Vec<_> = line
                    .chars() // Collecting all digits in the line.
                    .filter(|c| c.is_digit(10))
                    .collect();
                // Creating a string from the first and last digit.
                let combined_digits = format!("{}{}", digits[0], digits[digits.len() - 1]);
                // Parsing the string as a number, defaulting to 0 on error.
                match combined_digits.parse::<u32>() {
                    Ok(num) => Some(num),
                    Err(_) => Some(0),
                }
            })
            .sum::<u32>() as i64 // Summing all numbers and casting to i64.
    }

    // Part 2 of the challenge: Handling spelled-out numbers.
    pub fn part_2(&self, input: &str) -> i64 {
        let replacements = [
            // Mapping of spelled-out numbers to their digit forms.
            ("zero", "z0o"),
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "th3ree"),
            ("four", "fo4r"),
            ("five", "fi5ve"),
            ("six", "si6x"),
            ("seven", "se7ven"),
            ("eight", "ei8ght"),
            ("nine", "ni9ne"),
        ];

        let mut modified_input = input.to_string(); // Copying the input for modification.
        for (word, replacement) in replacements.iter() {
            // Replacing spelled-out numbers.
            modified_input = modified_input.replace(word, replacement);
        }
        self.part_1(&modified_input) // Reusing part_1 logic on modified input.
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day01 of AoC 2023.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2023_day01_part_1() {
        use crate::solutions::aoc2023::Day01;
        let day01 = Day01 {
            day: 1,
            year: 23,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        "#;
        let expected = 142;
        assert_eq!(day01.part_1(input), expected); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day01 of AoC 2023.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2023_day01_part_2() {
        use crate::solutions::aoc2023::Day01;
        let day01 = Day01 {
            day: 1,
            year: 23,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "#;
        let expected = 281;
        assert_eq!(day01.part_2(input), expected); // Asserts if the function output matches the expected result.
    }
}
