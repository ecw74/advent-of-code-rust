use super::day15::Day15;
use std::collections::HashSet;

impl Day15 {
    /// Generalized input parsing for both parts
    fn parse_input<F>(
        input: &str,
        transform_map: F, // Function to transform the map for Part 1 or Part 2
    ) -> (Vec<Vec<char>>, Vec<(isize, isize)>, (isize, isize))
    where
        F: Fn(usize, usize, char, &mut (isize, isize)) -> Vec<char>,
    {
        // Filter out empty lines and split into relevant sections
        let lines: Vec<&str> = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect();

        let mut start = (0, 0); // Starting position of the robot

        // Parse the 2D map
        let map: Vec<Vec<char>> = lines
            .iter()
            .take_while(|line| line.starts_with('#') && line.ends_with('#')) // Only lines of the map
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(|(x, ch)| transform_map(x, y, ch, &mut start))
                    .collect()
            })
            .collect();

        // Parse the movement commands
        let commands: Vec<(isize, isize)> = lines
            .iter()
            .skip_while(|line| line.starts_with('#') && line.ends_with('#')) // Only movement commands
            .flat_map(|line| {
                line.chars().map(|ch| match ch {
                    '<' => (-1, 0), // Left
                    '^' => (0, -1), // Up
                    '>' => (1, 0),  // Right
                    'v' => (0, 1),  // Down
                    _ => panic!("Unknown direction: {}", ch),
                })
            })
            .collect();

        (map, commands, start)
    }

    /// Transformation function for Part 1 (single-width boxes)
    fn transform_part_1(x: usize, y: usize, ch: char, start: &mut (isize, isize)) -> Vec<char> {
        if ch == '@' {
            *start = (x as isize, y as isize); // Record the robot's starting position
        }
        vec![ch] // Return the character unmodified
    }

    /// Transformation function for Part 2 (double-width boxes)
    fn transform_part_2(x: usize, y: usize, ch: char, start: &mut (isize, isize)) -> Vec<char> {
        match ch {
            '#' => vec!['#', '#'], // Double walls
            '.' => vec!['.', '.'], // Double empty spaces
            'O' => vec!['[', ']'], // Boxes become wider (represented by brackets)
            '@' => {
                *start = ((x * 2) as isize, y as isize); // Adjust robot's starting position
                vec!['@', '.'] // Robot expands to match the scaled map
            }
            _ => panic!("Unexpected character in map: {}", ch),
        }
    }

    /// Parse input for Part 1
    pub fn parse_input_part_1(
        input: &str,
    ) -> (Vec<Vec<char>>, Vec<(isize, isize)>, (isize, isize)) {
        Self::parse_input(input, Self::transform_part_1)
    }

    /// Parse input for Part 2
    pub fn parse_input_part_2(
        input: &str,
    ) -> (Vec<Vec<char>>, Vec<(isize, isize)>, (isize, isize)) {
        Self::parse_input(input, Self::transform_part_2)
    }

    /// Solve Part 1
    pub fn part_1(&self, input: &str) -> String {
        let (mut map, commands, mut pos) = Self::parse_input_part_1(input);

        for dir in commands {
            let mut next = (pos.0 + dir.0, pos.1 + dir.1);

            // Keep pushing boxes until a wall or free space is found
            while map[next.1 as usize][next.0 as usize] == 'O' {
                next = (next.0 + dir.0, next.1 + dir.1);
            }

            if map[next.1 as usize][next.0 as usize] == '.' {
                // Move cells backwards to simulate the push
                while next != pos {
                    let prev = (next.0 - dir.0, next.1 - dir.1);
                    map[next.1 as usize][next.0 as usize] = map[prev.1 as usize][prev.0 as usize];
                    next = prev;
                }
                // Clear the robot's previous position
                map[pos.1 as usize][pos.0 as usize] = '.';
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
        }

        // Sum the GPS coordinates of all boxes
        map.iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == 'O') // Only box positions
                    .map(move |(col, _)| 100u64 * row as u64 + col as u64) // Calculate GPS
            })
            .flatten()
            .sum::<u64>()
            .to_string()
    }

    /// Expand connected regions of wide boxes (Part 2)
    fn expand_connected_regions(
        map: &Vec<Vec<char>>,
        pos: (isize, isize),
        dir: (isize, isize),
    ) -> Option<HashSet<(isize, isize)>> {
        let mut boxes = Vec::from([pos]);
        let mut all_boxes = HashSet::from([pos]);

        while let Some(curr) = boxes.pop() {
            let next = (curr.0 + dir.0, curr.1 + dir.1);
            if all_boxes.contains(&next) {
                continue;
            }
            let c = map[next.1 as usize][next.0 as usize];
            match c {
                '.' => {} // Found an empty edge
                ']' | '[' => {
                    // Add the box part to the frontier
                    boxes.push(next);
                    all_boxes.insert(next);
                    if dir.1 != 0 {
                        let other = if c == '[' {
                            (next.0 + 1, next.1)
                        } else {
                            (next.0 - 1, next.1)
                        };
                        boxes.push(other);
                        all_boxes.insert(other);
                    }
                }
                _ => return None,
            }
        }
        Some(all_boxes)
    }

    /// Solve Part 2
    pub fn part_2(&self, input: &str) -> String {
        let (mut map, commands, mut pos) = Self::parse_input_part_2(input);
        let mut scratch_grid = map.clone();

        for dir in commands {
            if let Some(boxes) = Self::expand_connected_regions(&map, pos, dir) {
                // Backup and move all connected cells
                for &cell in &boxes {
                    scratch_grid[cell.1 as usize][cell.0 as usize] =
                        map[cell.1 as usize][cell.0 as usize];
                    map[cell.1 as usize][cell.0 as usize] = '.'; // Clear old positions
                }
                for &cell in &boxes {
                    let new_c = (cell.0 + dir.0, cell.1 + dir.1);
                    map[new_c.1 as usize][new_c.0 as usize] =
                        scratch_grid[cell.1 as usize][cell.0 as usize];
                }
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }
        }

        // Sum the GPS coordinates of all wide boxes
        map.iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == '[') // Left edge of wide boxes
                    .map(move |(col, _)| 100u64 * row as u64 + col as u64)
            })
            .flatten()
            .sum::<u64>()
            .to_string()
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
        let input = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "#;
        assert_eq!(day15.part_1(input), "10092"); // Asserts if the function output matches the expected result.
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
        let input = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "#;
        assert_eq!(day15.part_2(input), "9021"); // Asserts if the function output matches the expected result.
    }
}
