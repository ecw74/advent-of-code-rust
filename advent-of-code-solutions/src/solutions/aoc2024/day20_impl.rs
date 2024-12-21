use super::day20::Day20;
use crate::utils::point::Point;
use pathfinding::prelude::{bfs, Grid};

impl Day20 {
    /// Parses the map and identifies the grid, start (S), and end (E) points.
    fn parse_map(map: &str) -> (Grid, Point<i16>, Point<i16>) {
        let lines: Vec<&str> = map.lines().filter(|line| !line.trim().is_empty()).collect();
        let height = lines.len();
        let width = lines
            .first()
            .expect("Map must have at least one line")
            .len();

        let mut grid = Grid::new(width, height);
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point {
                    x: x as i16,
                    y: y as i16,
                };
                match c {
                    'S' => {
                        start = point;
                        grid.add_vertex((x, y));
                    }
                    'E' => {
                        end = point;
                        grid.add_vertex((x, y));
                    }
                    '#' => {} // Wall, skip adding.
                    _ => {
                        grid.add_vertex((x, y));
                    }
                }
            }
        }

        (grid, start, end)
    }

    /// Finds valid cheats based on the condition function.
    fn find_cheats<F>(path: &[Point<i16>], level: u32, condition: F) -> u32
    where
        F: Fn(i16, usize, u32) -> bool,
    {
        let mut count = 0u32;

        for (i, &position) in path.iter().enumerate() {
            for j in (i + 3)..path.len() {
                let target = path[j];
                let steps = j - i;
                let dist = position.manhattan(target);

                if condition(dist, steps, level) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Generalized solution for both parts of the puzzle.
    pub fn solve<F>(input: &str, level_fn: fn(usize) -> u32, condition: F) -> String
    where
        F: Fn(i16, usize, u32) -> bool,
    {
        let (map, start, end) = Self::parse_map(input);

        let neighbors = |&p: &Point<i16>| -> Vec<Point<i16>> {
            map.neighbours((p.x as usize, p.y as usize))
                .into_iter()
                .map(|(x, y)| Point {
                    x: x as i16,
                    y: y as i16,
                })
                .collect()
        };

        if let Some(path) = bfs(&start, neighbors, |&p| p == end) {
            let level = level_fn(input.len());
            return Self::find_cheats(&path, level, condition).to_string();
        }

        "".to_string()
    }

    /// Part 1 solution: finds cheats saving 2 picoseconds.
    pub fn part_1(&self, input: &str) -> String {
        Self::solve(
            input,
            |len| if len < 1000 { 20 } else { 100 },
            |dist, steps, level| dist == 2 && (steps as u32 - 2 >= level),
        )
    }

    /// Part 2 solution: finds cheats with updated rules allowing up to 20 picoseconds.
    pub fn part_2(&self, input: &str) -> String {
        Self::solve(
            input,
            |len| if len < 1000 { 50 } else { 100 },
            |dist, steps, level| {
                dist <= 20 && dist < steps as i16 && (steps as u32 - dist as u32 >= level)
            },
        )
    }
}

mod test {
    #[test]
    /// Test for Part 1.
    fn test_aoc2024_day20_part_1() {
        use crate::solutions::aoc2024::Day20;
        let day20 = Day20 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
        "#;
        assert_eq!(day20.part_1(input), "5");
    }

    #[test]
    /// Test for Part 2.
    fn test_aoc2024_day20_part_2() {
        use crate::solutions::aoc2024::Day20;
        let day20 = Day20 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
        "#;
        assert_eq!(day20.part_2(input), "285");
    }
}
