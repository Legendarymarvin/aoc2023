use std::collections::HashSet;
use itertools::Itertools;
use crate::Direction::{South, West, East, North};
use crate::Tile::{Ground, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Start, Vertical};

fn main() {
    let input = include_str!("./inputs/input10");
    let (part1, part2) = solve(input);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn solve(p0: &str) -> (i32, i32) {
    let (start, matrix) = read_pipe_matrix(p0);
    let origin = Node { x: start.1 as i32, y: start.0 as i32 };
    let mut tile = Start;
    let mut dir = get_start_dir(origin, &matrix);
    let mut current = origin.clone();
    let mut counter = 0;
    let mut left: HashSet<Node> = HashSet::new();
    let mut right: HashSet<Node> = HashSet::new();
    let mut looping: HashSet<Node> = HashSet::new();
    loop {
        counter += 1;
        looping.insert(current);
        // for part 2
        fill_up_left_and_right(&dir, &tile, &current, &mut left, &mut right);

        current = dir.do_move(current);
        tile = matrix[current.y as usize][current.x as usize];
        dir = match tile {
            Start => dir,
            Vertical => dir,
            Horizontal => dir,
            NorthEast => if dir == South { East } else { North },
            NorthWest => if dir == South { West } else { North },
            SouthWest => if dir == North { West } else { South },
            SouthEast => if dir == North { East } else { South },
            Ground => dir
        };

        if current == origin {
            break;
        }

        if counter >= 400000 {
            break;
        }
    }

    // part 2
    let max = matrix.len() as i32;
    left = filter_loop_and_outside_matrix(&mut left, &looping, max);
    right = filter_loop_and_outside_matrix(&mut right, &looping, max);
    mark_unmarked_as_right_or_left(&matrix, &mut left, &mut right, &looping);

    let count_inside = if left.contains(&Node { x: 0, y: 0 }) { right.len() } else {left.len()};
    ((counter) / 2, count_inside as i32)
}

fn get_start_dir(p0: Node, matrix: &Vec<Vec<Tile>>) -> Direction {
    return if vec![Horizontal, SouthWest, NorthWest].contains(&p0.east().get_tile(matrix)) {
        East
    } else if vec![Horizontal, SouthEast, NorthEast].contains(&p0.west().get_tile(matrix)) {
        West
    } else if vec![Vertical, NorthEast, NorthWest].contains(&p0.south().get_tile(matrix)) {
        South
    } else if vec![Vertical, SouthEast, SouthWest].contains(&p0.north().get_tile(matrix)) {
        North
    } else {
        panic!("Where to?");
    };
}

fn mark_unmarked_as_right_or_left(matrix: &Vec<Vec<Tile>>, left: &mut HashSet<Node>, right: &mut HashSet<Node>, looping: &HashSet<Node>) {
    let max: i32 = matrix.len() as i32;

    let mut unmarked = vec![];
    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            let node = &Node { x: x as i32, y: y as i32 };
            if !left.contains(node) && !right.contains(node) && !looping.contains(node) {
                unmarked.push(*node);
            }
        }
    }

    loop {
        let node = match unmarked.pop() {
            None => { break; }
            Some(n) => { n }
        };
        if node.x > 0 && left.contains(&node.west()) {
            left.insert(node);
        }
        if node.x > 0 && right.contains(&node.west()) {
            right.insert(node);
        }
        if node.x < max && left.contains(&node.east()) {
            left.insert(node);
        }
        if node.x < max && right.contains(&node.east()) {
            right.insert(node);
        }

        if node.y > 0 && left.contains(&node.north()) {
            left.insert(node);
        }
        if node.y > 0 && right.contains(&node.north()) {
            right.insert(node);
        }
        if node.y < max && left.contains(&node.south()) {
            left.insert(node);
        }
        if node.y < max && right.contains(&node.south()) {
            right.insert(node);
        }
    }
}

fn filter_loop_and_outside_matrix(set: &mut HashSet<Node>, looping: &HashSet<Node>, max: i32) -> HashSet<Node> {
    set.iter()
        .filter(|n| !looping.contains(n))
        .filter(|n| n.x > 0 && n.y > 0 && n.x < max && n.y < max)
        .unique()
        .cloned()
        .collect()
}

fn fill_up_left_and_right(dir: &Direction, tile: &Tile, current: &Node, left: &mut HashSet<Node>, right: &mut HashSet<Node>) {
    match dir {
        East => {
            match tile {
                Vertical => {
                    right.insert(current.south());
                    left.insert(current.north());
                }
                NorthEast => {
                    right.insert(current.west());
                    right.insert(current.south());
                }
                SouthEast => {
                    left.insert(current.west());
                    left.insert(current.north());
                }
                _ => {}
            }
        }
        West => {
            match tile {
                Vertical => {
                    right.insert(current.north());
                    left.insert(current.south());
                }
                NorthWest => {
                    left.insert(current.east());
                    left.insert(current.south());
                }
                SouthWest => {
                    right.insert(current.east());
                    right.insert(current.north());
                }
                _ => {}
            }
        }
        South => {
            match tile {
                Horizontal => {
                    right.insert(current.west());
                    left.insert(current.east());
                }
                SouthWest => {
                    left.insert(current.east());
                    left.insert(current.north());
                }
                SouthEast => {
                    right.insert(current.west());
                    right.insert(current.north());
                }
                _ => {}
            }
        }
        North => {
            match tile {
                Horizontal => {
                    right.insert(current.east());
                    left.insert(current.west());
                }
                NorthWest => {
                    right.insert(current.east());
                    right.insert(current.south());
                }
                NorthEast => {
                    left.insert(current.west());
                    left.insert(current.south());
                }
                _ => {}
            }
        }
    }
}

fn read_pipe_matrix(p0: &str) -> ((usize, usize), Vec<Vec<Tile>>) {
    let mut matrix = vec![];
    let mut start: (usize, usize) = (0, 0);
    for (i, line) in p0.lines().enumerate() {
        let mut x = vec![];
        for (j, char) in line.chars().enumerate() {
            let tile: Tile = match char {
                '|' => Vertical,
                '-' => Horizontal,
                'L' => NorthEast,
                'J' => NorthWest,
                '7' => SouthWest,
                'F' => SouthEast,
                '.' => Ground,
                'S' => {
                    start = (i, j);
                    Start
                }
                _ => panic!("Wtf is this: {}", char)
            };
            x.push(tile);
        }
        matrix.push(x);
    }
    (start, matrix)
}


#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
enum Tile {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node {
    x: i32,
    y: i32,
}

impl Node {
    fn west(&self) -> Node {
        Node { x: self.x - 1, y: self.y }
    }
    fn east(&self) -> Node {
        Node { x: self.x + 1, y: self.y }
    }
    fn north(&self) -> Node {
        Node { x: self.x, y: self.y - 1 }
    }
    fn south(&self) -> Node {
        Node { x: self.x, y: self.y + 1 }
    }
    fn get_tile(&self, matrix: &Vec<Vec<Tile>>) -> Tile {
        matrix[self.y as usize][self.x as usize]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    East,
    West,
    South,
    North,
}

impl Direction {
    fn do_move(&self, point: Node) -> Node {
        match self {
            South => point.south(),
            East => point.east(),
            West => point.west(),
            North => point.north()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve};

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input);
        assert_eq!(result.0, 8);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"
    };
        input
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input10");
        let part1 = solve(input);
        assert_eq!(part1.0, 6838);
        assert_eq!(part1.1, 451);
    }

}
