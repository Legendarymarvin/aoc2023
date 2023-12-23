use std::collections::{HashMap, HashSet};
use crate::Thing::{Forrest, Nothing, SlopeDown, SlopeLeft, SlopeRight};

fn main() {
    let input = include_str!("./inputs/input23");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {} , Part2: {} ", part1, part2);
}


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Thing {
    Forrest,
    Nothing,
    SlopeDown,
    //SlopeUp, I don't think it actually exists?
    SlopeRight,
    SlopeLeft,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Tile {
    kind: Thing,
    pos: (usize, usize),
}

impl Tile {
    fn from_char(ch: char) -> Thing {
        match ch {
            '#' => Forrest,
            '.' => Nothing,
            '>' => SlopeRight,
            '<' => SlopeLeft,
            'v' => SlopeDown,
            _ => panic!("NO!")
        }
    }
}

#[derive(Debug)]
struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Tile>,
    edges: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Graph {}


pub fn solve(input: &str, part2: bool) -> usize {
    let (length, graph, _matrix) = parse_graph(input, part2);
    let mut max_length = 0;
    let mut current_length = 0;
    let mut visited = HashSet::new();
    let start = (1usize, 0usize);
    let end = (length - 2, length - 1);

    depth_first_search(start, end, &graph, &mut visited, &mut current_length, &mut max_length);

    max_length
}

fn depth_first_search(start: (usize, usize), end: (usize, usize), graph: &Graph,
                      visited: &mut HashSet<(usize, usize)>, current_length: &mut usize, max_length: &mut usize) {
    if start == end {
        *max_length = *current_length.max(max_length);
        dbg!(max_length);
        return;
    }

    visited.insert(start);

    for &neighbour in graph.edges.get(&start).unwrap_or(&Vec::new()) {
        if !visited.contains(&neighbour) {
            *current_length += 1;
            depth_first_search(neighbour, end, graph, visited, current_length, max_length);
            *current_length -= 1;
        }
    }

    visited.remove(&start);
}

fn parse_graph(input: &str, part2: bool) -> (usize, Graph, HashMap<(usize, usize), Thing>) {
    let mut nodes = Vec::new();
    let mut edges = HashMap::new();
    let mut matrix = HashMap::new();
    let length = input.lines().next().unwrap().len();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let thing = Tile::from_char(ch);
            matrix.insert((x, y), thing);
            if thing != Forrest {
                nodes.push(Tile { kind: thing, pos: (x, y) });
                edges.insert((x, y), vec![]);
            }
        }
    }

    for tile in &nodes {
        edges.insert(tile.pos, get_neighbours(tile, &matrix, length, part2));
    }
    (length, Graph { nodes, edges }, matrix)
}

fn get_neighbours(tile: &Tile, matrix: &HashMap<(usize, usize), Thing>, length: usize, part2: bool) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let (x, y) = tile.pos;

    if part2 {
        match tile.kind {
            Nothing | SlopeDown | SlopeLeft | SlopeRight => {
                if y > 0 && matrix.get(&(x, y - 1)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x, y - 1)) }
                if y < length && matrix.get(&(x, y + 1)).unwrap_or(&crate::Thing::Forrest) != &Forrest { neighbours.push((x, y + 1)) }
                if x > 0 && matrix.get(&(x - 1, y)).unwrap_or(&crate::Thing::Forrest) != &crate::Thing::Forrest { neighbours.push((x - 1, y)) }
                if x < length && matrix.get(&(x + 1, y)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x + 1, y)) }
            }
            _ => ()
        }
    } else {
        match tile.kind {
            Nothing => {
                if y > 0 && matrix.get(&(x, y - 1)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x, y - 1)) }
                if y < length && matrix.get(&(x, y + 1)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x, y + 1)) }
                if x > 0 && matrix.get(&(x - 1, y)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x - 1, y)) }
                if x < length && matrix.get(&(x + 1, y)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x + 1, y)) }
            }
            SlopeDown => {
                // the forrest check is probably unnecessary for slopes.
                if y < length && matrix.get(&(x, y + 1)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x, y + 1)) }
            }
            SlopeRight => {
                if x < length && matrix.get(&(x + 1, y)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x + 1, y)) }
            }
            SlopeLeft => {
                if x > 0 && matrix.get(&(x - 1, y)).unwrap_or(&Forrest) != &Forrest { neighbours.push((x - 1, y)) }
            }
            _ => ()
        }
    }

    neighbours
}


#[cfg(test)]
mod tests {
    use crate::{solve};

    use indoc::indoc;


    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let (result) = solve(input, false);
        assert_eq!(result, 94);
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result, 154);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {
"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
    };
        input
    }

    // #[test]
    // fn still_working() {
    //     let input = include_str!("./inputs/input22");
    //     let (part1, part2) = solve(input, );
    //     assert_eq!(part1, 517);
    //     assert_eq!(part2, 61276);
    // }
}
