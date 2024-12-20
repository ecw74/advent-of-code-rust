use super::day16::Day16;
use crate::utils::point::Point;
use hashbrown::HashSet;
use pathfinding::prelude::astar_bag;

/// Represents the state of the Reindeer in the maze.
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Reindeer {
    pos: Point, // Current position in the maze.
    dir: Point, // Current direction (e.g., (1, 0) for East).
}

/// Contains the solution logic for Day 16.
impl Day16 {
    /// Parses the maze input into a map grid, start position, and end position.
    fn parse_map(input: &str) -> (Vec<Vec<char>>, Point, Point) {
        let mut map = Vec::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        // Process each line in the input.
        for (row_idx, line) in input.lines().filter(|s| !s.trim().is_empty()).enumerate() {
            let mut row = Vec::new();
            for (col_idx, ch) in line.trim().chars().enumerate() {
                match ch {
                    'S' => {
                        start = Point {
                            x: col_idx as i16,
                            y: row_idx as i16,
                        }
                    }
                    'E' => {
                        end = Point {
                            x: col_idx as i16,
                            y: row_idx as i16,
                        }
                    }
                    '#' | '.' => {} // Valid map characters.
                    _ => panic!(
                        "Invalid character '{}' in map at ({}, {})",
                        ch, row_idx, col_idx
                    ),
                }
                row.push(ch);
            }
            map.push(row);
        }

        (map, start, end)
    }

    /// Generates all valid moves for the given reindeer state.
    fn get_successors(r: &Reindeer, map: &Vec<Vec<char>>) -> Vec<(Reindeer, u32)> {
        let mut potential_positions = vec![];

        // Move forward.
        let ahead = r.pos + r.dir;
        if Self::is_valid(&ahead, map) {
            potential_positions.push((
                Reindeer {
                    pos: ahead,
                    dir: r.dir,
                },
                1u32,
            ));
        }

        // Turn left and move.
        let left = r.pos + r.dir.rotate_ccw();
        if Self::is_valid(&left, map) {
            potential_positions.push((
                Reindeer {
                    pos: left,
                    dir: r.dir.rotate_ccw(),
                },
                1001u32,
            ));
        }

        // Turn right and move.
        let right = r.pos + r.dir.rotate_cw();
        if Self::is_valid(&right, map) {
            potential_positions.push((
                Reindeer {
                    pos: right,
                    dir: r.dir.rotate_cw(),
                },
                1001u32,
            ));
        }

        // Move backward.
        let behind = r.pos - r.dir;
        if Self::is_valid(&behind, map) {
            potential_positions.push((
                Reindeer {
                    pos: behind,
                    dir: r.dir.rotate_cw().rotate_cw(),
                },
                2001u32,
            ));
        }

        potential_positions
    }

    #[inline]
    /// Checks if a point is valid (within bounds and not a wall).
    fn is_valid(p: &Point, map: &Vec<Vec<char>>) -> bool {
        (0..map[0].len()).contains(&(p.x as usize))
            && (0..map.len()).contains(&(p.y as usize))
            && map[p.y as usize][p.x as usize] != '#'
    }

    /// Solves Part 1: Finds the minimum cost to traverse the maze.
    pub fn part_1(&self, input: &str) -> String {
        let (map, start, end) = Self::parse_map(input);
        let reindeer = Reindeer {
            pos: start,
            dir: Point { x: 1, y: 0 },
        };

        // Use A* search to find the minimum cost.
        let (_solution, cost) = astar_bag(
            &reindeer,
            |r| Self::get_successors(r, &map),
            |r| r.pos.manhattan(end) as u32,
            |r| r.pos == end,
        )
        .unwrap();

        cost.to_string()
    }

    /// Solves Part 2: Counts the tiles in the optimal paths.
    pub fn part_2(&self, input: &str) -> String {
        let (map, start, end) = Self::parse_map(input);
        let reindeer = Reindeer {
            pos: start,
            dir: Point { x: 1, y: 0 },
        };

        // Use A* search to find all optimal paths and their tiles.
        let (solution, _cost) = astar_bag(
            &reindeer,
            |r| Self::get_successors(r, &map),
            |r| r.pos.manhattan(end) as u32,
            |r| r.pos == end,
        )
        .unwrap();

        // Collect all tiles from the optimal paths.
        let all_points: HashSet<Point> = solution.fold(HashSet::default(), |mut set, rs| {
            set.extend(rs.iter().map(|r| r.pos));
            set
        });

        all_points.len().to_string()
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day16 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day16_part_1() {
        use crate::solutions::aoc2024::Day16;
        let day16 = Day16 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        "#;
        assert_eq!(day16.part_1(input), "7036"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day16 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day16_part_2() {
        use crate::solutions::aoc2024::Day16;
        let day16 = Day16 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        "#;
        assert_eq!(day16.part_2(input), "45"); // Asserts if the function output matches the expected result.
    }
}
