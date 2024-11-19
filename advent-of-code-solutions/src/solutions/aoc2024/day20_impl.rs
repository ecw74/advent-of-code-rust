use super::day20::Day20;

impl Day20 {
    pub fn part_1(&self, _input: &str) -> String {
        todo!()
    }
    pub fn part_2(&self, _input: &str) -> String {
        todo!()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day20 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day20_part_1() {
        use crate::solutions::aoc2024::Day20;
        let day20 = Day20 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        assert_eq!(day20.part_1(input), "TBD"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day20 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day20_part_2() {
        use crate::solutions::aoc2024::Day20;
        let day20 = Day20 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        assert_eq!(day20.part_2(input), "TBD"); // Asserts if the function output matches the expected result.
    }
}
