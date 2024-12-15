use super::day15::Day15;

impl Day15 {
    fn parse_input_part_1(input: &str) -> (Vec<Vec<char>>, Vec<char>, (usize, usize)) {
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

    fn parse_input_part_2(input: &str) -> (Vec<Vec<char>>, Vec<char>, (usize, usize)) {
        // Filter out empty lines and split into relevant sections
        let lines: Vec<&str> = input
            .lines()
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines
            .collect();

        let mut start = (0, 0);

        let map: Vec<Vec<char>> = lines
            .iter()
            .take_while(|line| line.starts_with('#') && line.ends_with('#'))
            .enumerate() // Erhalte die Zeilenindizes für die Position von '@'
            .map(|(y, line)| {
                line.chars()
                    .enumerate() // Erhalte die Spaltenindizes für die Position von '@'
                    .flat_map(|(x, ch)| match ch {
                        '#' => vec!['#', '#'], // Wand wird horizontal verdoppelt
                        '.' => vec!['.', '.'], // Leerraum wird horizontal verdoppelt
                        'O' => vec!['[', ']'], // Hindernis wird horizontal verdoppelt
                        '@' => {
                            start = (x * 2, y); // Position von '@' speichern (x verdoppeln)
                            vec!['@', '.'] // '@' wird zu '..' umgewandelt
                        }
                        _ => panic!("Unexpected character in map: {}", ch), // Fehler für unerwartete Zeichen
                    })
                    .collect() // Sammle die modifizierten Zeichen in eine Zeile
            })
            .collect(); // Sammle alle Zeilen in die 2D-Map

        // Parse the movement commands: Remaining lines after the map
        let commands: Vec<char> = lines
            .iter()
            .skip_while(|line| line.starts_with('#') && line.ends_with('#'))
            .flat_map(|line| line.chars().map(|ch| ch))
            .collect();

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
            if let Some(&(_, (dr, dc))) = directions.iter().find(|&&(dir, _)| dir == movement) {
                let next_pos = (
                    (robot_pos.0 as isize + dr) as usize,
                    (robot_pos.1 as isize + dc) as usize,
                );

                // Prüfen, ob die nächste Position innerhalb der Grenzen liegt
                if next_pos.0 >= rows || next_pos.1 >= cols || map[next_pos.0][next_pos.1] == '#' {
                    continue; // Ungültige Bewegung
                }

                // Hindernis: Einzelzeichen 'O' verschieben
                if map[next_pos.0][next_pos.1] == 'O' {
                    if !Self::shift_obstacles(next_pos, (dr, dc), map) {
                        continue; // Bewegung abbrechen, falls Hindernis nicht verschoben werden kann
                    }
                }

                // Hindernis: Paar '[' und ']' verschieben
                if map[next_pos.0][next_pos.1] == '[' {
                    if next_pos.1 < cols && map[next_pos.0][next_pos.1 + 1] == ']' {
                        if !Self::shift_obstacle_pair(next_pos, (dr, dc), map) {
                            continue; // Bewegung abbrechen, falls Paar nicht verschoben werden kann
                        }
                    }
                }

                // Hindernis: Falls Roboter ']' trifft
                if map[next_pos.0][next_pos.1] == ']' {
                    if next_pos.1 > 0 && map[next_pos.0][next_pos.1 - 1] == '[' {
                        if !Self::shift_obstacle_pair((next_pos.0, next_pos.1 - 1), (dr, dc), map) {
                            continue; // Bewegung abbrechen
                        }
                    } else {
                        continue; // Ungültiges Zeichen, überspringen
                    }
                }

                // Bewegung des Roboters
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
        let (mut map, commands, start) = Self::parse_input_part_1(input);

        Self::move_robot(start, &mut map, &commands);

        map.iter().enumerate()
            .map(|(row, line)| line.iter().enumerate()
                .filter(|&(_, &c)| c == 'O')
                .map(move |(col, _)| 100u64 * row as u64 + col as u64))
            .flatten()
            .sum::<u64>().to_string()
    }

    /// Push an obstacle pair ('[' and ']') in the given direction using DFS.
    /// If the pair can't be moved, return false.
    /// Verschiebt ein Hindernispaar ('[' und ']') rekursiv.
    /// Gibt `false` zurück, wenn das Hindernispaar nicht verschoben werden kann.
    fn shift_obstacle_pair(
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
        let next_pair_pos = (
            next_pos.0,
            (next_pos.1 as isize + 1) as usize,
        );

        // Prüfen, ob die aktuelle Position ein gültiges Paar ist
        if pos.1 + 1 >= cols || map[pos.0][pos.1] != '[' || map[pos.0][pos.1 + 1] != ']' {
            return false; // Kein gültiges Paar
        }

        // Prüfen, ob die Zielpositionen innerhalb der Karte liegen und frei sind
        if next_pos.0 >= rows
            || next_pos.1 >= cols
            || next_pair_pos.1 >= cols
            || map[next_pos.0][next_pos.1] != '.'
            || map[next_pair_pos.0][next_pair_pos.1] != '.'
        {
            return false; // Zielpositionen blockiert oder außerhalb der Karte
        }

        // Falls ein weiteres Hindernispaar im Weg ist, verschiebe es rekursiv
        if map[next_pos.0][next_pos.1] == '[' && map[next_pos.0][next_pair_pos.1] == ']' {
            if !Self::shift_obstacle_pair(next_pos, dir, map) {
                return false; // Nachfolgendes Hindernis konnte nicht verschoben werden
            }
        }

        // Verschiebe das aktuelle Paar
        map[next_pos.0][next_pos.1] = '[';
        map[next_pair_pos.0][next_pair_pos.1] = ']';
        map[pos.0][pos.1] = '.';
        map[pos.0][pos.1 + 1] = '.';

        true
    }

    pub fn part_2(&self, input: &str) -> String {
        let (mut map, commands, start) = Self::parse_input_part_2(input);

        println!("Map Input");
        for r in map.iter_mut() {
            println!("{}", r.iter().collect::<String>());
        }

        println!("Robot Movements");
        println!("{}", commands.iter().collect::<String>());

        Self::move_robot(start, &mut map, &commands);

        println!("Map Output");
        for r in map.iter_mut() {
            println!("{}", r.iter().collect::<String>());
        }

        "".to_string()

        // map.iter().enumerate()
        //     .map(|(row, line)| line.iter().enumerate()
        //         .filter(|&(_, &c)| c == 'O')
        //         .map(move |(col, _)| 100u64 * row as u64 + col as u64))
        //     .flatten()
        //     .sum::<u64>().to_string()
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
