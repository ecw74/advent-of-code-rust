use super::day17::Day17;

/// Represents the 3-Bit Computer's State.
/// This includes registers (A, B, C), the program input, an instruction pointer,
/// and the output buffer for collected results.
pub struct State {
    pub a: u32,             // Register A
    pub b: u32,             // Register B
    pub c: u32,             // Register C
    pub input: Vec<usize>,  // Program input (sequence of 3-bit opcodes and operands)
    pub inst: usize,        // Instruction pointer, tracks the current program position
    pub output: Vec<usize>, // Storage for output values produced by the 'out' instruction
}

impl State {
    /// Parses the input string and initializes the `State` structure.
    /// Extracts initial register values (A, B, C) and the program instructions.
    pub fn from_input(input: &str) -> Self {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut input_vec = Vec::new();

        // Parse each line of input to extract registers and program instructions.
        for line in input.lines() {
            if line.starts_with("Register A:") {
                a = Self::parse_value(line);
            } else if line.starts_with("Register B:") {
                b = Self::parse_value(line);
            } else if line.starts_with("Register C:") {
                c = Self::parse_value(line);
            } else if line.starts_with("Program:") {
                input_vec = Self::parse_program(line);
            }
        }

        // Initialize the state with parsed values.
        State {
            a,
            b,
            c,
            input: input_vec,
            inst: 0,            // Instruction pointer starts at 0
            output: Vec::new(), // Initially empty output buffer
        }
    }

    /// Runs the program by processing each instruction until the halt condition is met.
    pub fn run_program(&mut self) {
        // Continue execution until the instruction pointer is out of bounds.
        while self.inst < self.input.len() {
            let opcode = self.input[self.inst]; // Current instruction (opcode)
            let operand = self.input.get(self.inst + 1).cloned().unwrap_or(0) as u8; // Operand

            // Match the opcode to the corresponding instruction and execute it.
            match opcode {
                0 => self.adv(operand), // ADV: Divide A by 2^operand
                1 => self.bxl(operand), // BXL: XOR B with operand
                2 => self.bst(operand), // BST: Set B = operand % 8
                3 => {
                    // JNZ: Jump if A != 0
                    if self.jnz(operand) {
                        continue; // Skip instruction pointer increment if jump occurs
                    }
                }
                4 => self.bxc(operand), // BXC: XOR B with C
                5 => self.out(operand), // OUT: Output operand % 8
                6 => self.bdv(operand), // BDV: Divide A by 2^operand, store result in B
                7 => self.cdv(operand), // CDV: Divide A by 2^operand, store result in C
                _ => break,             // Invalid opcode halts the program
            }

            self.inst += 2; // Move to the next instruction (each instruction is 2 values)
        }
    }

    /// Converts the program output (a list of numbers) into a comma-separated string.
    pub fn to_string(&self) -> String {
        self.output
            .iter()
            .map(|num| num.to_string()) // Convert each number to a string
            .collect::<Vec<String>>() // Collect into a vector of strings
            .join(",") // Join the vector into a single string
    }

    /// Resolves the value of a combo operand based on its type.
    /// Combo operands map to literal values (0-3) or register values (4-6).
    fn resolve_operand(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32, // Literal values 0-3
            4 => self.a,             // Register A
            5 => self.b,             // Register B
            6 => self.c,             // Register C
            _ => 0,                  // Operand 7 is reserved
        }
    }

    /// Opcode 0 - ADV: Divide register A by 2^operand.
    fn adv(&mut self, operand: u8) {
        let denom = 2_u32.pow(self.resolve_operand(operand) as u32);
        self.a /= denom;
    }

    /// Opcode 1 - BXL: XOR register B with the literal operand.
    fn bxl(&mut self, operand: u8) {
        self.b ^= operand as u32;
    }

    /// Opcode 2 - BST: Set register B to operand % 8.
    fn bst(&mut self, operand: u8) {
        self.b = self.resolve_operand(operand) % 8;
    }

    /// Opcode 3 - JNZ: Jump to operand if register A is not zero.
    /// Returns `true` if a jump occurred.
    fn jnz(&mut self, operand: u8) -> bool {
        if self.a != 0 {
            self.inst = self.resolve_operand(operand) as usize;
            true
        } else {
            false
        }
    }

    /// Opcode 4 - BXC: XOR register B with register C (ignores operand).
    fn bxc(&mut self, _operand: u8) {
        self.b ^= self.c;
    }

    /// Opcode 5 - OUT: Output operand % 8.
    fn out(&mut self, operand: u8) {
        let value = (self.resolve_operand(operand) % 8) as u8;
        self.output.push(value as usize);
    }

    /// Opcode 6 - BDV: Divide register A by 2^operand and store the result in register B.
    fn bdv(&mut self, operand: u8) {
        let denom = 2_u32.pow(self.resolve_operand(operand));
        self.b = self.a / denom;
    }

    /// Opcode 7 - CDV: Divide register A by 2^operand and store the result in register C.
    fn cdv(&mut self, operand: u8) {
        let denom = 2_u32.pow(self.resolve_operand(operand));
        self.c = self.a / denom;
    }

    /// Parses a line containing a register value and returns the numeric value.
    fn parse_value(line: &str) -> u32 {
        line.split(':')
            .nth(1) // Extract the part after the colon
            .and_then(|val| val.trim().parse::<u32>().ok()) // Convert to u32
            .unwrap_or(0) // Default to 0 if parsing fails
    }

    /// Parses the program input (comma-separated numbers) into a vector of `usize`.
    fn parse_program(line: &str) -> Vec<usize> {
        line.split(':')
            .nth(1) // Extract the part after the colon
            .unwrap_or("")
            .split(',') // Split numbers by commas
            .filter_map(|num| num.trim().parse::<usize>().ok()) // Parse numbers into usize
            .collect()
    }
}

impl Day17 {
    /// Solves Part 1: Executes the program and returns the output as a string.
    pub fn part_1(&self, input: &str) -> String {
        let mut state = State::from_input(input);
        state.run_program();
        state.to_string()
    }

    /// Solves Part 2: Finds the lowest positive value for A that satisfies the program conditions.
    pub fn part_2(&self, input: &str) -> String {
        let state = State::from_input(input);
        let program = state.input;
        let mut a = 0;

        // Extract values from the BXL (opcode 1) instructions
        let [bxl1, bxl2] = program
            .chunks(2)
            .filter(|p| p[0] == 1) // Only consider BXL instructions
            .map(|p| p[1]) // Extract operands
            .take(2)
            .collect::<Vec<usize>>()[..]
        else {
            panic!("Did not find two bxl commands")
        };

        for n in 1..=program.len() {
            let target = program[program.len() - n..].to_vec();

            let mut new_a = a << 3;
            loop {
                let mut digits = Vec::new();
                let mut test_a = new_a;

                while test_a != 0 {
                    let mut b = test_a & 0x07;
                    b = b ^ bxl1;
                    let c = test_a >> b;
                    b = b ^ c;
                    b = b ^ bxl2;
                    test_a >>= 3;

                    let test_digit = b & 0x07;
                    if test_digit != *target.get(digits.len()).unwrap() {
                        break;
                    }
                    digits.push(test_digit);
                }

                if digits == target {
                    a = new_a;
                    break;
                }
                new_a += 1;
            }
        }

        a.to_string()
    }
}

mod test {

    #[test]
    /// Test for part 1 of Day17 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day17_part_1() {
        use crate::solutions::aoc2024::day17_impl::State;
        let mut input = r#"
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
        "#;
        let mut state = State::from_input(input);
        state.run_program();
        assert_eq!(state.b, 1);

        input = r#"
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
        "#;
        state = State::from_input(input);
        state.run_program();
        assert_eq!(state.to_string(), "0,1,2");

        input = r#"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
        "#;
        state = State::from_input(input);
        state.run_program();
        assert_eq!(state.a, 0);
        assert_eq!(state.to_string(), "4,2,5,6,7,7,7,7,3,1,0");

        input = r#"
Register A: 0
Register B: 29
Register C: 0

Program: 1,7
        "#;
        state = State::from_input(input);
        state.run_program();
        assert_eq!(state.b, 26);

        input = r#"
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
        "#;
        state = State::from_input(input);
        state.run_program();
        assert_eq!(state.b, 44354);

        input = r#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
        "#;
        state = State::from_input(input);
        state.run_program();
        assert_eq!(state.a, 0);
        assert_eq!(state.to_string(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    /// Test for part 2 of Day17 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day17_part_2() {
        use crate::solutions::aoc2024::Day17;
        let day17 = Day17 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
Register A: 52042868
Register B: 0
Register C: 0

Program: 2,4,1,7,7,5,0,3,4,4,1,7,5,5,3,0
        "#;
        assert_eq!(day17.part_2(input), "267265166222235"); // Asserts if the function output matches the expected result.
    }
}
