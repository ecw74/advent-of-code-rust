use super::day15::Day15;

impl Day15 {
    pub fn part_1(&self, _input: &str) -> i64 {
        todo!()
    }
    pub fn part_2(&self, _input: &str) -> i64 {
        todo!()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day15 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day15_part_1() {
        use crate::solutions::aoc2024::Day15;
        let day15 = Day15 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        let expected = 42;
        assert_eq!(day15.part_1(input), expected); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day15 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day15_part_2() {
        use crate::solutions::aoc2024::Day15;
        let day15 = Day15 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        let expected = 42;
        assert_eq!(day15.part_2(input), expected); // Asserts if the function output matches the expected result.
    }
}
