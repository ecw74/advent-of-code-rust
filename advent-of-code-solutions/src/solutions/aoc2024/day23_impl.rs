use super::day23::Day23;
use hashbrown::{HashMap, HashSet};

impl Day23 {
    /// Parses the input string into an adjacency list representation of the network.
    /// Each line in the input represents an undirected connection between two nodes.
    /// Returns a `HashMap` where keys are nodes and values are sets of connected neighbors.
    fn parse_network(input: &str) -> HashMap<String, HashSet<String>> {
        let mut adjacency_list: HashMap<String, HashSet<String>> = HashMap::new();

        for line in input.lines().filter(|line| !line.trim().is_empty()) {
            // Split each line into two nodes and trim whitespace.
            if let Some((node1, node2)) = line.split_once('-') {
                let node1 = node1.trim().to_string();
                let node2 = node2.trim().to_string();

                // Add node2 as a neighbor of node1.
                adjacency_list
                    .entry(node1.clone())
                    .or_insert_with(HashSet::new)
                    .insert(node2.clone());

                // Add node1 as a neighbor of node2.
                adjacency_list
                    .entry(node2)
                    .or_insert_with(HashSet::new)
                    .insert(node1);
            }
        }

        adjacency_list
    }

    /// Finds all triangles (cliques of size 3) in the graph.
    /// A triangle consists of three nodes (u, v, w) that are mutually connected.
    /// Returns a list of triangles as tuples of node names in lexicographical order.
    fn find_triangles(graph: &HashMap<String, HashSet<String>>) -> Vec<(String, String, String)> {
        let mut triangles = Vec::new();
        // Iterate over all nodes and their neighbors in the graph.
        for (u, neighbors_u) in graph {
            for v in neighbors_u {
                if let Some(neighbors_v) = graph.get(v) {
                    // Find common neighbors of u and v.
                    let common_neighbors: HashSet<_> =
                        neighbors_u.intersection(neighbors_v).collect();
                    for w in common_neighbors {
                        // Check if any node in the triangle starts with 't'.
                        if u.starts_with('t') || v.starts_with('t') || w.starts_with('t') {
                            // Create a sorted representation of the triangle.
                            let mut triangle = vec![u.clone(), v.clone(), w.clone()];
                            triangle.sort();
                            let tuple = (
                                triangle[0].clone(),
                                triangle[1].clone(),
                                triangle[2].clone(),
                            );
                            if !triangles.contains(&tuple) {
                                triangles.push(tuple);
                            }
                        }
                    }
                }
            }
        }

        triangles
    }

    /// Part 1: Counts the number of triangles in the graph where at least one node starts with 't'.
    /// Uses `parse_network` to build the graph and `find_triangles` to identify valid triangles.
    pub fn part_1(&self, input: &str) -> String {
        let network = Day23::parse_network(input);
        let triangles = Self::find_triangles(&network);
        triangles.len().to_string() // Return the count of triangles as a string.
    }

    /// Finds the largest clique (fully connected subgraph) in an undirected graph.
    /// Returns the largest clique as a sorted vector of node names.
    pub fn find_max_clique(graph: &HashMap<String, HashSet<String>>) -> Vec<String> {
        // Convert graph references to &str for efficient set operations.
        let graph_ref: HashMap<&str, HashSet<&str>> = graph
            .iter()
            .map(|(key, neighbors)| {
                let neighbors_ref: HashSet<&str> = neighbors.iter().map(String::as_str).collect();
                (key.as_str(), neighbors_ref)
            })
            .collect();

        let mut all_cliques: Vec<HashSet<&str>> = Vec::new();
        let mut r = HashSet::new(); // Current clique.
        let p: HashSet<_> = graph_ref.keys().cloned().collect(); // All nodes are candidates initially.
        let x = HashSet::new(); // No nodes are excluded initially.

        // Use the Bron-Kerbosch algorithm with pivot optimization to find maximal cliques.
        Self::bron_kerbosch(&graph_ref, &mut r, p, x, &mut all_cliques);

        // Identify the largest clique from all found cliques.
        let max_clique = all_cliques
            .into_iter()
            .max_by_key(|clique| clique.len())
            .unwrap_or_default();

        // Convert the largest clique to a sorted `Vec<String>`.
        let mut sorted_clique: Vec<String> =
            max_clique.into_iter().map(|s| s.to_string()).collect();
        sorted_clique.sort();
        sorted_clique
    }

    /// Bron-Kerbosch algorithm with pivot optimization to find all maximal cliques in the graph.
    fn bron_kerbosch<'a>(
        graph: &HashMap<&'a str, HashSet<&'a str>>, // Adjacency list of the graph.
        r: &mut HashSet<&'a str>,                   // Current clique.
        p: HashSet<&'a str>,                        // Potential candidates.
        x: HashSet<&'a str>,                        // Excluded nodes.
        cliques: &mut Vec<HashSet<&'a str>>,        // List of all maximal cliques.
    ) {
        if p.is_empty() && x.is_empty() {
            cliques.push(r.clone()); // If no candidates or exclusions remain, save the current clique.
            return;
        }

        // Select a pivot node to reduce recursive calls.
        let pivot = p.union(&x).next().cloned().unwrap_or_default();

        // Process nodes not connected to the pivot.
        let non_neighbors: HashSet<_> = p.difference(&graph[&pivot]).cloned().collect();

        for node in non_neighbors {
            let neighbours = &graph[node];
            let p2: HashSet<_> = p.intersection(neighbours).cloned().collect();
            let x2: HashSet<_> = x.intersection(neighbours).cloned().collect();

            r.insert(node);
            Self::bron_kerbosch(graph, r, p2, x2, cliques);
            r.remove(node);
        }
    }

    /// Part 2: Finds the largest clique in the graph and returns its nodes joined by commas.
    pub fn part_2(&self, input: &str) -> String {
        let network = Day23::parse_network(input);
        let max_clique = Self::find_max_clique(&network);
        max_clique.join(",") // Join nodes with commas to create the password.
    }
}

mod test {
    #[test]
    /// Test for Part 1 of Day23.
    /// Validates the number of triangles containing a node starting with 't'.
    fn test_aoc2024_day23_part_1() {
        use crate::solutions::aoc2024::Day23;
        let day23 = Day23 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
        "#;
        assert_eq!(day23.part_1(input), "7"); // Checks if the output matches the expected result.
    }

    #[test]
    /// Test for Part 2 of Day23.
    /// Validates the largest clique and its formatted string output.
    fn test_aoc2024_day23_part_2() {
        use crate::solutions::aoc2024::Day23;
        let day23 = Day23 {
            day: 0,
            year: 0,
            desc: "".to_string(),
            code: "".to_string(),
        };
        let input = r#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
        "#;
        assert_eq!(day23.part_2(input), "co,de,ka,ta"); // Validates the correct largest clique output.
    }
}
