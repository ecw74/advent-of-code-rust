use super::day02::Day02;

use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    id: i64,
    reveals: Vec<HashMap<String, i64>>,
}

impl Day02 {
    fn parse_input(input: &str) -> Vec<Game> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Skip empty lines
            .map(|line| {
                let parts: Vec<&str> = line.split(": ").collect();
                let id = parts[0].trim_start_matches("Game ").parse::<i64>().unwrap();
                let reveals = parts[1]
                    .split("; ")
                    .map(|reveal| {
                        reveal
                            .split(", ")
                            .map(|cube| {
                                let parts: Vec<&str> = cube.split(' ').collect();
                                let count = parts[0].parse::<i64>().unwrap();
                                let color = parts[1].to_string();
                                (color, count)
                            })
                            .collect()
                    })
                    .collect();
                Game { id, reveals }
            })
            .collect()
    }

    // Solves Part 1 of the puzzle: sum the IDs of games that are possible with fixed cube limits.
    pub fn part_1(&self, input: &str) -> i64 {
        let games = Self::parse_input(input);
        let max_cubes = (12, 13, 14); // (red, green, blue)
        let (max_red, max_green, max_blue) = max_cubes;

        games
            .iter()
            .filter_map(|game| {
                if game.reveals.iter().all(|reveal| {
                    let red = reveal.get("red").unwrap_or(&0);
                    let green = reveal.get("green").unwrap_or(&0);
                    let blue = reveal.get("blue").unwrap_or(&0);

                    *red <= max_red && *green <= max_green && *blue <= max_blue
                }) {
                    Some(game.id)
                } else {
                    None
                }
            })
            .sum()
    }

    // Solves Part 2 of the puzzle: sum the powers of the minimum required cube sets.
    pub fn part_2(&self, _input: &str) -> i64 {
        let games = Self::parse_input(_input);
        games
            .iter()
            .map(|game| {
                // Find the maximum count of each color across all reveals
                let mut min_cubes = HashMap::new();

                for reveal in &game.reveals {
                    for (color, &count) in reveal {
                        *min_cubes.entry(color.clone()).or_insert(0) =
                            (*min_cubes.get(color.as_str()).unwrap_or(&0)).max(count);
                    }
                }

                // Calculate power as red * green * blue
                let red = *min_cubes.get("red").unwrap_or(&0);
                let green = *min_cubes.get("green").unwrap_or(&0);
                let blue = *min_cubes.get("blue").unwrap_or(&0);

                let power = red * green * blue;

                power
            })
            .sum()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day02 for AoC 2023.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2023_day02_part_1() {
        use crate::solutions::aoc2023::Day02;
        let day02 = Day02 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let expected = 8;
        assert_eq!(day02.part_1(input), expected); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day02 for AoC 2023.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2023_day02_part_2() {
        use crate::solutions::aoc2023::Day02;
        let day02 = Day02 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        let expected = 2286;
        assert_eq!(day02.part_2(input), expected); // Asserts if the function output matches the expected result.
    }
}
