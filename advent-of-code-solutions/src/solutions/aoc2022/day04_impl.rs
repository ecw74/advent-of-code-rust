use super::day04::Day04;

impl Day04 {
    /// Helper function to parse a line into two ranges.
    /// Returns a tuple of ranges (start1, end1) and (start2, end2).
    fn parse_ranges(line: &str) -> ((i32, i32), (i32, i32)) {
        let pair: Vec<&str> = line.split(",").collect();

        let range1: (i32, i32) = {
            let nums: Vec<i32> = pair[0]
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            (nums[0], nums[1])
        };

        let range2: (i32, i32) = {
            let nums: Vec<i32> = pair[1]
                .split("-")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            (nums[0], nums[1])
        };

        (range1, range2)
    }

    /// Part 1: Counts the number of assignment pairs where one range fully contains the other.
    pub fn part_1(&self, _input: &str) -> i64 {
        _input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let ((start1, end1), (start2, end2)) = Self::parse_ranges(line);

                // Check if one range fully contains the other.
                if (start2 <= start1 && end1 <= end2) || (start1 <= start2 && end2 <= end1) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    /// Part 2: Counts the number of assignment pairs that overlap at all.
    pub fn part_2(&self, _input: &str) -> i64 {
        _input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let ((start1, end1), (start2, end2)) = Self::parse_ranges(line);

                // Check if the ranges overlap at all.
                if (start2 <= end1 && start2 >= start1) || (start1 <= end2 && start1 >= start2) {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

mod test {
    #[test]
    /// Test for Part 1: Validates the function with example input for fully contained ranges.
    fn test_aoc2022_day04_part_1() {
        use crate::solutions::aoc2022::Day04;
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "#;
        assert_eq!(day04.part_1(input), 2); // Expect 2 pairs with full containment.
    }

    #[test]
    /// Test for Part 2: Validates the function with example input for overlapping ranges.
    fn test_aoc2022_day04_part_2() {
        use crate::solutions::aoc2022::Day04;
        let day04 = Day04 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
        "#;
        assert_eq!(day04.part_2(input), 4); // Expect 4 pairs with any overlap.
    }
}
