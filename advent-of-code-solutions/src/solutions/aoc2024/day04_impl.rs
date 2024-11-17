use super::day04::Day04;

impl Day04 {
    pub fn part_1(&self, _input: &str) -> i64 {
        todo!()
    }
    pub fn part_2(&self, _input: &str) -> i64 {
        todo!()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day04 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day04_part_1() {
        use crate::solutions::aoc2024::Day04;
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        let expected = 42;
        assert_eq!(day04.part_1(input), expected); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day04 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day04_part_2() {
        use crate::solutions::aoc2024::Day04;
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        let expected = 42;
        assert_eq!(day04.part_2(input), expected); // Asserts if the function output matches the expected result.
    }
}
