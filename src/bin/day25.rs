use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("./inputs/input25");
    let part1 = solve(input);
    println!("Part1: {} , Part2: {} ", part1, part1);
    Ok(())
}

pub fn solve(input: &str) -> usize {
    let graph = parse_graph(input);
    let mut rng = thread_rng();
    let mut paths_with_counts = HashMap::new();
    let keys: Vec<String> = graph.keys().cloned().collect();

    let mut potential_solutions = Vec::new();
    // This monte carlo approach doesn't necessarily return the correct one every time,
    // so let's just do it five times and take the majority candidate, if it works for spaceships, it works for this.
    // One could also do the main loop a lot more often, but this is actually a lot faster.
    for _ in 0..=5 {
        for _ in 0..100 {
            let (a, b): (String, String) = keys.choose_multiple(&mut rng, 2).cloned().collect_tuple().unwrap();
            if let Some(path) = find_bfs_path(&graph, a.clone(), b.clone()) {
                for window in path.windows(2) {
                    let edge = (window[0].clone(), window[1].clone());
                    *paths_with_counts.entry(edge).or_insert(0) += 1;
                }
            }
        }

        let mut temp = graph.clone();
        let candidates = get_candidates(&mut paths_with_counts);

        cut_connections(&mut temp, &candidates);

        let x1 = candidates.into_iter().next().expect("How can there be no candidates?");
        let first = count_graph(&temp, x1.0);
        let second = count_graph(&temp, x1.1);
        potential_solutions.push(first * second);
    }
    potential_solutions.iter()
        .fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val).unwrap()

    // //graphviz solution
    //let dot = graph_to_dot(&graph);
    //println!("{}", dot);
    //save_to_file(&dot, "day25.dot").unwrap();
    //generate_graph_image().unwrap();
    // //look at image, insert magic solution.
    // let x1 = ("ljh".to_string(), "tbg".to_string());
    // let magic_solution = vec![x1.clone(), ("mfs".to_string(), "ffv".to_string()), ("mnh".to_string(), "qnv".to_string())];
    // cut_connections(&mut graph, magic_solution);
    // let first = count_graph(&graph, x1.0);
    // let second = count_graph(&graph, x1.1);
    //
    // first * second
}

fn get_candidates(paths_with_counts: &mut HashMap<(String, String), i32>) -> Vec<(String, String)> {
    let sorted_by_counts: Vec<_> = paths_with_counts.into_iter().sorted_by(|a, b| b.1.cmp(&a.1)).collect();
    let mut candidates: Vec<(String, String)> = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();
    for ((x, y), _value) in sorted_by_counts.iter() {
        // we currently have combinations with both directions, only need it once.
        let reverse = (y.clone(), x.clone());
        if !seen.contains(&reverse) {
            candidates.push((x.clone(), y.clone()));
        }
        seen.insert((x.clone(), y.clone()));
        if candidates.len() >= 3 {
            break;
        }
    }
    candidates
}

#[allow(dead_code)]
fn save_to_file(content: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
fn graph_to_dot(graph: &HashMap<String, HashSet<String>>) -> String {
    let mut dot = String::from("graph G {\n");
    for (node, edges) in graph {
        for edge in edges {
            dot.push_str(&format!("    \"{}\" -- \"{}\";\n", node, edge));
        }
    }
    dot.push('}');
    dot
}

#[allow(dead_code)]
fn generate_graph_image() -> io::Result<()> {
    let status = Command::new("dot")
        .arg("-Tsvg")
        .arg("-Kneato")
        .arg("day25.dot")
        .arg("-o")
        .arg("day25.svg")
        .status()?;

    if !status.success() {
        eprintln!("Failed to generate graph image.");
        if let Some(code) = status.code() {
            eprintln!("Process exited with code {}", code);
        } else {
            eprintln!("Process terminated by signal");
        }
    }

    Ok(())
}

// remnants of solutions long past
// fn get_edges(graph: &HashMap<String, HashSet<String>>) -> Vec<(String, String)> {
//     let mut edges = Vec::new();
//     for (node, neighbors) in graph.iter() {
//         for neighbor in neighbors {
//             edges.push((node.clone(), neighbor.clone()));
//         }
//     }
//     edges
// }
//
// fn is_disconnected(graph: &HashMap<String, HashSet<String>>) -> bool {
//     if graph.is_empty() {
//         return false;
//     }
//
//     let mut visited = HashSet::new();
//     let mut components = 0;
//
//     for node in graph.keys() {
//         if !visited.contains(node) {
//             components += 1;
//             dfs(node, graph, &mut visited);
//         }
//     }
//
//     components > 1
// }
//
// fn dfs(node: &str, graph: &HashMap<String, HashSet<String>>, visited: &mut HashSet<String>) {
//     visited.insert(node.to_string());
//     if let Some(neighbors) = graph.get(node) {
//         for neighbor in neighbors {
//             if !visited.contains(neighbor) {
//                 dfs(neighbor, graph, visited);
//             }
//         }
//     }
//}


fn count_graph(graph: &HashMap<String, HashSet<String>>, p1: String) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(p1.to_string());
    queue.push_back(p1.to_string());

    while let Some(node) = queue.pop_front() {
        // Visit all adjacent nodes
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if visited.insert(neighbor.clone()) {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    visited.len()
}

fn parse_graph(p0: &str) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();
    p0.lines().for_each(|line| {
        let (origin, conns) = line.split_once(": ").expect("Should have a : delimiter");
        conns.split(" ").for_each(|c| {
            graph.entry(String::from(origin)).or_insert_with(HashSet::new).insert(String::from(c));
            graph.entry(String::from(c)).or_insert_with(HashSet::new).insert(String::from(origin));
        })
    });
    graph
}

fn cut_connections(p0: &mut HashMap<String, HashSet<String>>, p1: &Vec<(String, String)>) {
    p1.iter().for_each(|pair| {
        if let Some(set) = p0.get_mut(&pair.0) {
            set.remove(&pair.1);
        }
        if let Some(set) = p0.get_mut(&pair.1) {
            set.remove(&pair.0);
        }
    });
}

fn find_bfs_path(p0: &HashMap<String, HashSet<String>>, p1: String, p2: String) -> Option<Vec<String>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((p1.clone(), vec![p1]));

    while let Some((curr, path)) = queue.pop_front() {
        if curr == p2 {
            return Some(path);
        }

        if let Some(next_nodes) = p0.get(&curr) {
            for n in next_nodes {
                if !visited.contains(n) {
                    visited.insert(n);
                    let mut new_path = path.clone();
                    new_path.push(n.clone());
                    queue.push_back((n.clone(), new_path));
                }
            }
        }
    }

    None
}



#[cfg(test)]
mod tests {
    use crate::{solve};

    use indoc::indoc;


    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let (result) = solve(input);
        assert_eq!(result, 54);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {
"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
    };
        input
    }

}
