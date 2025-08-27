use petgraph::graph::{NodeIndex, UnGraph};
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

fn find_triangles(g: &UnGraph<String, ()>) -> Vec<(String, String, String)> {
    let mut triangles = Vec::new();
    let mut seen = HashSet::new();

    for u in g.node_indices() {
        for v in g.neighbors(u) {
            if v <= u {
                continue;
            }
            for w in g.neighbors(v) {
                if w <= v {
                    continue;
                }
                if g.contains_edge(u, w) {
                    let mut triple = vec![g[u].clone(), g[v].clone(), g[w].clone()];
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

fn get_largest_clique(g: &UnGraph<String, ()>) -> Vec<String> {
    let mut best = Vec::new();
    let mut r = HashSet::new();
    let mut p: HashSet<_> = g.node_indices().collect();
    let mut x = HashSet::new();

    bron_kerbosch(g, &mut r, &mut p, &mut x, &mut best);

    best.into_iter().map(|idx| g[idx].clone()).collect()
}

/// Bron–Kerbosch recursive algorithm for finding maximum clique
/// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch(
    g: &UnGraph<String, ()>,
    r: &mut HashSet<NodeIndex>,
    p: &mut HashSet<NodeIndex>,
    x: &mut HashSet<NodeIndex>,
    best: &mut Vec<NodeIndex>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > best.len() {
            *best = r.iter().cloned().collect();
        }
        return;
    }

    let pivot = p.iter().chain(x.iter()).next().cloned();

    let candidates: Vec<NodeIndex> = if let Some(u) = pivot {
        p.difference(&g.neighbors(u).collect()).cloned().collect()
    } else {
        p.iter().cloned().collect()
    };

    for v in candidates {
        r.insert(v);

        let mut p_new = &*p & &g.neighbors(v).collect();
        let mut x_new = &*x & &g.neighbors(v).collect();

        bron_kerbosch(g, r, &mut p_new, &mut x_new, best);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

pub fn part_2(input: &str) -> String {
    let graph = parse_graph(input);

    let largest = get_largest_clique(&graph);

    let mut largest = largest;
    largest.sort();

    return largest.join(",");
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
