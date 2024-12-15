use std::collections::VecDeque;
use super::day15::Day15;

impl Day15 {
    fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>, (usize, usize)) {
        // Filter out empty lines and split into relevant sections
        let lines: Vec<&str> = input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines
            .collect();

        // Parse the 2D map by finding lines that start and end with '#'
        let map: Vec<Vec<char>> = lines
            .iter()
            .take_while(|line| line.starts_with('#') && line.ends_with('#'))
            .map(|line| line.chars().collect())
            .collect();

        // Parse the movement commands: Remaining lines after the map
        let commands: Vec<char> = lines
            .iter()
            .skip_while(|line| line.starts_with('#') && line.ends_with('#'))
            .flat_map(|line| line.chars().map(|ch| ch))
            .collect();

        // Find the position of '@' in the map
        let mut start = (0, 0);
        for (row, line) in map.iter().enumerate() {
            if let Some(col) = line.iter().position(|&c| c == '@') {
                start = (row, col);
                break;
            }
        }

        (map, commands, start)
    }
    fn move_robot(
        start: (usize, usize),
        map: &mut Vec<Vec<char>>,
        moves: &Vec<char>,
    ) {
        let directions: Vec<(char, (isize, isize))> = vec![
            ('^', (-1, 0)),  // Bewegung nach oben
            ('v', (1, 0)),   // Bewegung nach unten
            ('<', (0, -1)),  // Bewegung nach links
            ('>', (0, 1)),   // Bewegung nach rechts
        ];

        let rows = map.len();
        let cols = map[0].len();
        let mut robot_pos = start;

        for &movement in moves {
            // Finde die Bewegungsrichtung
            if let Some(&(_, (dr, dc))) = directions.iter().find(|&&(dir, _)| dir == movement) {
                let next_pos = (
                    (robot_pos.0 as isize + dr) as usize,
                    (robot_pos.1 as isize + dc) as usize,
                );

                // Überprüfen, ob die Bewegung innerhalb der Grenzen liegt
                if next_pos.0 >= rows || next_pos.1 >= cols || map[next_pos.0][next_pos.1] == '#' {
                    continue; // Bewegung ist ungültig
                }

                // Falls der Roboter auf ein Hindernis stößt, Hindernisse verschieben (DFS)
                if map[next_pos.0][next_pos.1] == 'O' {
                    if !Self::shift_obstacles(next_pos, (dr, dc), map) {
                        continue; // Wenn Hindernisse nicht verschoben werden können, abbrechen
                    }
                }

                // Aktualisiere die Karte: Roboter bewegt sich
                map[robot_pos.0][robot_pos.1] = '.';
                map[next_pos.0][next_pos.1] = '@';
                robot_pos = next_pos;
            }
        }
    }

    fn shift_obstacles(
        pos: (usize, usize),
        dir: (isize, isize),
        map: &mut Vec<Vec<char>>,
    ) -> bool {
        let rows = map.len();
        let cols = map[0].len();

        let next_pos = (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        );

        // Prüfen, ob die nächste Position innerhalb der Grenzen liegt
        if next_pos.0 >= rows || next_pos.1 >= cols || map[next_pos.0][next_pos.1] == '#' {
            return false; // Hindernis kann nicht verschoben werden
        }

        // Wenn sich ein weiteres Hindernis vor dem aktuellen Hindernis befindet, verschiebe es rekursiv
        if map[next_pos.0][next_pos.1] == 'O' {
            if !Self::shift_obstacles(next_pos, dir, map) {
                return false;
            }
        }

        // Verschiebe das Hindernis
        map[next_pos.0][next_pos.1] = 'O';
        map[pos.0][pos.1] = '.';
        true
    }

    pub fn part_1(&self, input: &str) -> String {
        let (mut map, commands, start) = Self::parse_input(input);

        Self::move_robot(start, &mut map, &commands);

        map.iter().enumerate()
            .map(|(row, line)| line.iter().enumerate()
                .filter(|&(_, &c)| c == 'O')
                .map(move |(col, _)| 100u64 * row as u64 + col as u64))
            .flatten()
            .sum::<u64>().to_string()
    }
    pub fn part_2(&self, _input: &str) -> String {
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
        let input = r#""#;
        assert_eq!(day15.part_2(input), "TBD"); // Asserts if the function output matches the expected result.
    }
}
