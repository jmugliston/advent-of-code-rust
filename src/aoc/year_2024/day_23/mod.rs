use itertools::Itertools;
use petgraph::algo::maximal_cliques;
use petgraph::graph::UnGraph;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

use crate::utils::parsing;
use crate::Part;

const EXAMPLE_FILE: &str = "./src/aoc/year_2024/day_23/input/example.txt";
const INPUT_FILE: &str = "./src/aoc/year_2024/day_23/input/input.txt";

pub fn main(part: Part, example: bool) -> Result<(), Box<dyn Error>> {
    let input_file = if example { EXAMPLE_FILE } else { INPUT_FILE };

    let contents = fs::read_to_string(input_file)?;

    let res = match part {
        Part::One => part_1(&contents),
        Part::Two => part_2(&contents),
    };

    println!("{}", res);
    Ok(())
}

fn parse_graph(input: &str) -> UnGraph<String, ()> {
    let raw = parsing::read_lines(&input);
    let pairs: Vec<(String, String)> = raw
        .iter()
        .filter_map(|x| {
            let mut parts = x.split('-').map(|s| s.to_string());
            match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            }
        })
        .collect();

    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut node_indices = std::collections::HashMap::new();

    for (a, b) in &pairs {
        let idx_a = *node_indices
            .entry(a.clone())
            .or_insert_with(|| graph.add_node(a.clone()));
        let idx_b = *node_indices
            .entry(b.clone())
            .or_insert_with(|| graph.add_node(b.clone()));
        graph.add_edge(idx_a, idx_b, ());
    }

    graph
}

/// Find all triangles in the graph (3 nodes are all connected to each other)
fn find_triangles(g: &UnGraph<String, ()>) -> Vec<(String, String, String)> {
    let mut triangles = Vec::new();
    let mut seen = HashSet::new();

    for u in g.node_indices() {
        for v in g.neighbors(u).filter(|&v| v > u) {
            for w in g.neighbors(v).filter(|&w| w > v) {
                if g.contains_edge(u, w) {
                    let mut triple = [g[u].clone(), g[v].clone(), g[w].clone()];
                    triple.sort();
                    if seen.insert(triple.clone()) {
                        triangles.push((triple[0].clone(), triple[1].clone(), triple[2].clone()));
                    }
                }
            }
        }
    }

    triangles
}

pub fn part_1(input: &str) -> String {
    let graph = parse_graph(input);

    let triangles = find_triangles(&graph);

    let triangles_containing_t: Vec<(String, String, String)> = triangles
        .iter()
        .filter(|tri| tri.0.starts_with('t') || tri.1.starts_with('t') || tri.2.starts_with('t'))
        .cloned()
        .collect();

    return triangles_containing_t.len().to_string();
}

pub fn part_2(input: &str) -> String {
    let graph = parse_graph(input);

    let cliques = maximal_cliques(&graph);

    // Find the largest clique by number of nodes
    let largest_clique = cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap_or_default();

    // Sort the node names to get the password
    let password: String = largest_clique
        .into_iter()
        .map(|x| graph[x].clone())
        .sorted()
        .join(",");

    return password;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_1(&input);
                assert_eq!(result, "7");
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }

    #[test]
    fn test_part_2() {
        match fs::read_to_string(EXAMPLE_FILE) {
            Ok(input) => {
                let result = part_2(&input);
                assert_eq!(result, "co,de,ka,ta");
            }
            Err(e) => {
                eprintln!("Failed to read test input file: {}", e);
                panic!("Test input file missing or unreadable");
            }
        }
    }
}
