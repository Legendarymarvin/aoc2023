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
    let mut tile = get_start_tile(&matrix, origin);
    let mut dir: Direction = match tile {
        Vertical => South,
        Horizontal => East,
        NorthEast => East,
        NorthWest => North,
        SouthWest => West,
        SouthEast => South,
        _ => panic!()
    };
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

fn get_start_tile(matrix: &Vec<Vec<Tile>>, node: Node) -> Tile {
    let max = matrix.len() as i32;
    if node.y > 0 && vec![Vertical, SouthEast, SouthWest].contains(&node.north().get_tile(matrix)) {
        if [Vertical, NorthWest, NorthEast].contains(&node.south().get_tile(matrix)) {
            return Vertical
        } else if [Horizontal, NorthEast, SouthEast].contains(&node.west().get_tile(matrix)) {
            return NorthWest
        } else if [Horizontal, NorthWest,SouthWest].contains(&node.east().get_tile(matrix)) {
            return NorthEast
        }
    }
    if node.x > 0 && vec![Horizontal, NorthEast, SouthEast].contains(&node.west().get_tile(matrix)) {
        if vec![Horizontal, NorthWest, SouthWest].contains(&node.east().get_tile(matrix)) {
            return Horizontal
        } else if vec![Vertical, NorthEast, NorthWest].contains(&node.south().get_tile(matrix)) {
            return SouthWest
        } else if vec![Vertical, SouthWest, SouthEast].contains(&node.north().get_tile(matrix)) {
            return NorthWest
        }
    }
    if node.x < max && vec![Horizontal, NorthWest, SouthWest].contains(&node.east().get_tile(matrix)) {
       if vec![Vertical, NorthEast, NorthWest].contains(&node.south().get_tile(matrix)) {
           return SouthEast
       } else if [Vertical, SouthWest, SouthEast].contains(&node.north().get_tile(matrix)) {
           return NorthEast
       }
    }
    if node.y < max && vec![Vertical, NorthEast, NorthWest].contains(&node.south().get_tile(matrix)) {
        if vec![Horizontal, SouthEast, NorthEast].contains(&node.west().get_tile(matrix)) {
            return SouthWest
        } else if vec![Horizontal, SouthWest, NorthWest].contains(&node.east().get_tile(matrix)) {
            return SouthEast
        }
    }
    panic!("Uh oh...");
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
    use crate::{get_start_tile, Node, solve, Tile};

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


    fn create_test_matrix() -> Vec<Vec<Tile>> {
        vec![vec![Tile::Ground; 3]; 3]
    }

    #[test]
    fn test_horizontal() {
        let mut matrix = create_test_matrix();
        let node = Node { x: 1, y: 1 };

        matrix[0][1] = Tile::Vertical;
        matrix[2][1] = Tile::Vertical;

        assert_eq!(get_start_tile(&matrix, node), Tile::Vertical);

        matrix[0][1] = Tile::Vertical;
        matrix[2][1] = Tile::NorthEast;

        assert_eq!(get_start_tile(&matrix, node), Tile::Vertical);
    }

    #[test]
    fn test_vertical() {
        let mut matrix = create_test_matrix();
        let node = Node { x: 1, y: 1 };

        matrix[1][0] = Tile::Horizontal;
        matrix[1][2] = Tile::Horizontal;

        assert_eq!(get_start_tile(&matrix, node), Tile::Horizontal);

        matrix[1][0] = Tile::SouthEast;
        matrix[1][2] = Tile::NorthWest;
        assert_eq!(get_start_tile(&matrix, node), Tile::Horizontal);
    }

    #[test]
    fn test_north_east() {
        let mut matrix = create_test_matrix();
        let node = Node { x: 1, y: 1 };

        matrix[0][1] = Tile::Vertical;
        matrix[1][2] = Tile::Horizontal;

        assert_eq!(get_start_tile(&matrix, node), Tile::NorthEast);

        matrix[0][1] = Tile::SouthEast;
        matrix[1][2] = Tile::NorthWest;

        assert_eq!(get_start_tile(&matrix, node), Tile::NorthEast);
    }

    #[test]
    fn test_north_west() {
        let mut matrix = create_test_matrix();
        let node = Node { x: 1, y: 1 };

        matrix[0][1] = Tile::Vertical;
        matrix[1][0] = Tile::Horizontal;

        assert_eq!(get_start_tile(&matrix, node), Tile::NorthWest);

        matrix[0][1] = Tile::SouthWest;
        matrix[1][0] = Tile::SouthEast;

        assert_eq!(get_start_tile(&matrix, node), Tile::NorthWest);
    }

    #[test]
    fn test_south_east() {
        let mut matrix = create_test_matrix();
        let node = Node { x: 1, y: 1 };

        matrix[2][1] = Tile::Vertical;
        matrix[1][2] = Tile::Horizontal;

        assert_eq!(get_start_tile(&matrix, node), Tile::SouthEast);

        matrix[2][1] = Tile::Vertical;
        matrix[1][2] = Tile::NorthWest;

        assert_eq!(get_start_tile(&matrix, node), Tile::SouthEast);
    }

    #[test]
    fn test_south_west() {
        let mut matrix = create_test_matrix();
        let node = Node { x: 1, y: 1 };

        matrix[2][1] = Tile::Vertical;
        matrix[1][0] = Tile::Horizontal;

        assert_eq!(get_start_tile(&matrix, node), Tile::SouthWest);

        matrix[2][1] = Tile::NorthEast;
        matrix[1][0] = Tile::NorthEast;

        assert_eq!(get_start_tile(&matrix, node), Tile::SouthWest);
    }

    #[test]
    #[should_panic(expected = "Uh oh...")]
    fn test_panic_case() {
        let matrix = create_test_matrix();
        let node = Node { x: 1, y: 1 };

        get_start_tile(&matrix, node);
    }
}
