use super::day24::Day24;
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

/// Represents a logic gate in the system. Each gate takes two inputs, applies a specified operator,
/// and writes the result to an output wire. The gates simulate the malfunctioning boolean logic
/// described in the AoC story.
#[derive(Debug)]
struct Gate<'a> {
    input1: &'a str,
    input2: &'a str,
    operator: &'a str,
    output: &'a str,
}

/// Holds the parsed input of the circuit. This includes:
/// - `initial_values`: A map of wire names to their starting values (0 or 1).
/// - `gates`: A list of logic gates describing the connections and operations in the circuit.
#[derive(Debug)]
struct ParsedInput<'a> {
    initial_values: HashMap<&'a str, u8>,
    gates: Vec<Gate<'a>>,
}

impl Day24 {
    /// Parses the input string to extract initial wire values and gate definitions.
    /// The input format is as follows:
    /// - Lines of the form `wire: value` specify initial wire values.
    /// - Lines of the form `input1 OPERATOR input2 -> output` define gates.
    fn parse_input(input: &str) -> ParsedInput {
        let mut initial_values = HashMap::new();
        let mut gates = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue; // Ignore empty lines
            }

            if let Some((wire, value)) = line.split_once(":") {
                // Parse an initial wire value
                let wire = wire.trim();
                let value: u8 = value.trim().parse().expect("Invalid value");
                initial_values.insert(wire, value);
            } else if let Some((expression, output)) = line.split_once("->") {
                // Parse a gate definition
                let output = output.trim();
                let parts: Vec<&str> = expression.trim().split_whitespace().collect();

                if parts.len() == 3 {
                    // Gate format: "input1 OPERATOR input2 -> output"
                    let input1 = parts[0];
                    let operator = parts[1];
                    let input2 = parts[2];

                    gates.push(Gate {
                        input1,
                        input2,
                        operator,
                        output,
                    });
                } else {
                    panic!("Invalid gate format");
                }
            } else {
                panic!("Invalid line format");
            }
        }

        ParsedInput {
            initial_values,
            gates,
        }
    }

    /// Simulates the circuit by processing gates iteratively until all outputs are resolved.
    /// This function models the system described in the AoC story, where wires carry boolean values
    /// and gates compute their outputs based on input values and operators.
    fn simulate<'a>(input: &'a ParsedInput<'a>) -> HashMap<&'a str, u8> {
        let mut wire_values = input.initial_values.clone();
        let mut pending_gates: VecDeque<&Gate> = VecDeque::new();

        // Tracks which gates depend on each wire. This ensures efficient re-evaluation of gates.
        let mut dependents: HashMap<&str, Vec<&Gate>> = HashMap::new();

        // Initialize the dependents map and schedule all gates for processing.
        for gate in &input.gates {
            pending_gates.push_back(gate);
            dependents.entry(gate.input1).or_default().push(gate);
            dependents.entry(gate.input2).or_default().push(gate);
        }

        while let Some(gate) = pending_gates.pop_front() {
            // Check if both inputs for the gate are resolved
            if let (Some(&val1), Some(&val2)) =
                (wire_values.get(gate.input1), wire_values.get(gate.input2))
            {
                // Perform the operation specified by the gate
                let result = match gate.operator {
                    "AND" => val1 & val2,
                    "OR" => val1 | val2,
                    "XOR" => val1 ^ val2,
                    _ => panic!("Unsupported operator: {}", gate.operator),
                };

                // Update the output wire value only if it has changed
                let output_updated = if let Some(existing_val) = wire_values.get(gate.output) {
                    *existing_val != result
                } else {
                    true
                };

                if output_updated {
                    wire_values.insert(gate.output, result);

                    // Re-schedule dependent gates for evaluation
                    if let Some(dependent_gates) = dependents.get(gate.output) {
                        for &dependent_gate in dependent_gates {
                            pending_gates.push_back(dependent_gate);
                        }
                    }
                }
            } else {
                // Re-schedule the gate if inputs are not yet resolved
                pending_gates.push_back(gate);
            }
        }

        wire_values
    }

    /// Combines the values on wires starting with 'z' to form a binary number, interpreted as a decimal.
    /// The wires are sorted lexicographically (z00, z01, ...), and their binary values are combined
    /// to calculate the final output of the system.
    fn combine_bits(wire_values: HashMap<&str, u8>) -> u64 {
        // Filter keys that start with 'z' and collect them
        let mut filtered_keys: Vec<&str> = wire_values
            .keys()
            .filter(|key| key.starts_with('z'))
            .cloned()
            .collect();

        // Sort keys lexicographically (z00 < z01 < ...)
        filtered_keys.sort();

        // Combine values into a binary number
        let mut combined_value: u64 = 0;
        for (i, key) in filtered_keys.iter().enumerate() {
            if let Some(&value) = wire_values.get(key) {
                if value == 1 {
                    combined_value |= 1 << i; // Set the corresponding bit
                }
            }
        }

        combined_value
    }

    /// Solves Part 1 of the AoC problem by parsing the input, simulating the circuit, and combining
    /// the output bits to compute the final result as a decimal number.
    pub fn part_1(&self, input: &str) -> String {
        let logic_circuit = Self::parse_input(input);
        let wire_values = Self::simulate(&logic_circuit);
        Self::combine_bits(wire_values).to_string()
    }

    /// Identifies swapped wires for Part 2 of the AoC problem by analyzing gate dependencies and
    /// constraints to determine misconfigured outputs. Returns a sorted list of affected wire names.
    fn identify_swapped_wires(parsed_input: ParsedInput) -> String {
        let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
        for g in &parsed_input.gates {
            edges.entry(g.input1).or_default().push(g.output);
            edges.entry(g.input2).or_default().push(g.output);
        }

        let mut broken_nodes = HashSet::new();
        for g in &parsed_input.gates {
            // z nodes must be XOR (except for the last one, z45)
            if g.output.starts_with("z") && g.output != "z45" && g.operator != "XOR" {
                broken_nodes.insert(g.output);
            }
            // z nodes must not be inputs of other nodes
            if g.input1.starts_with("z") {
                broken_nodes.insert(g.input1);
            }
            if g.input2.starts_with("z") {
                broken_nodes.insert(g.input2);
            }

            // inputs of XOR nodes (except for z nodes) must be x and y nodes
            if g.operator == "XOR"
                && !g.output.starts_with("z")
                && !((g.input1.starts_with("x") && g.input2.starts_with("y"))
                    || (g.input1.starts_with("y") && g.input2.starts_with("x")))
            {
                broken_nodes.insert(g.output);
            }

            // XOR nodes (except z nodes) must always be input of exactly two
            // other nodes
            if g.operator == "XOR" && !g.output.starts_with("z") && edges[g.output].len() != 2 {
                broken_nodes.insert(g.output);
            }

            // AND nodes must always be input of exactly one other node (except
            // the very first one wired to x00 and y00)
            if g.operator == "AND"
                && !g.output.starts_with("z")
                && edges[g.output].len() != 1
                && !((g.input1 == "x00" && g.input2 == "y00")
                    || (g.input1 == "y00" && g.input2 == "x00"))
            {
                broken_nodes.insert(g.output);
            }
        }

        // this should be the answer:
        let mut swapped_wires = broken_nodes.into_iter().collect::<Vec<_>>();
        swapped_wires.sort();
        swapped_wires.join(",")
    }

    /// Solves Part 2 of the AoC problem by identifying swapped wires and returning their names in
    /// lexicographical order, as a comma-separated string.
    pub fn part_2(&self, input: &str) -> String {
        let logic_circuit = Self::parse_input(input);
        Self::identify_swapped_wires(logic_circuit)
    }
}

mod test {
    #[test]
    /// Test for part 1 of Day24 for AoC 2024.
    /// Ensures correct calculation of the sum of first and last digits in each line.
    fn test_aoc2024_day24_part_1() {
        use crate::solutions::aoc2024::Day24;
        let day24 = Day24 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
        "#;
        assert_eq!(day24.part_1(input), "2024"); // Asserts if the function output matches the expected result.
    }

    #[test]
    /// Test for part 2 of Day24 for AoC 2024.
    /// Verifies correct handling and sum of lines with spelled-out numbers.
    fn test_aoc2024_day24_part_2() {
        use crate::solutions::aoc2024::Day24;
        let day24 = Day24 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#""#;
        assert_eq!(day24.part_2(input), "TBD"); // Asserts if the function output matches the expected result.
    }
}
