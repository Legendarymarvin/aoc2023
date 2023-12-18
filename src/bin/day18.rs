use itertools::Itertools;
use crate::Direction::{East, North, South, West};

fn main() {
    let input = include_str!("./inputs/input18");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1, part2);
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    fn from_char(char: &str) -> Direction {
        match char {
            "U" => { North }
            "D" => { South }
            "R" => { East }
            "L" => { West }
            &_ => panic!("Oh no!")
        }
    }

    fn from_num_char(char: &str) -> Direction {
        match char {
            "3" => { North }
            "1" => { South }
            "0" => { East }
            "2" => { West }
            &_ => panic!("Oh no!")
        }
    }
}

#[derive(Debug)]
struct Operation {
    direction: Direction,
    steps: i32,
    // I thought we might end up caring about the colour when doing part1.
    //colour: &'o str,
}

fn solve(p0: &str, part2: bool) -> i64 {
    let ops = parse_ops(p0, part2);
    let (border_count, corners) = do_ops(&ops);

    let area = calculate_shoelace_area(&corners);
    let border = border_count / 2 + 1;
    area + border as i64
}

fn parse_ops(p0: &str, part2: bool) -> Vec<Operation> {
    if part2 {
        p0.lines()
            .map(|line| {
                let (_, _, part2_operation) = line.split(" ").into_iter().collect_tuple().unwrap();
                let metres = &part2_operation[2..=6];
                let dir = &part2_operation[7..=7];
                return Operation { direction: Direction::from_num_char(dir), steps: i32::from_str_radix(metres, 16).unwrap() };
            })
            .collect::<Vec<Operation>>()
    } else {
        p0.lines()
            .map(|line| {
                let (dir, step, _part2_cares) = line.split(" ").into_iter().collect_tuple().unwrap();
                return Operation { direction: Direction::from_char(dir), steps: step.parse::<i32>().unwrap() };
            })
            .collect::<Vec<Operation>>()
    }
}

fn do_ops(ops: &Vec<Operation>) -> (i32, Vec<(i32, i32)>) {
    let mut current = (0, 0);
    let mut counter = 0;
    let corners = ops.iter().map(|op| {
        counter += op.steps;
        match op.direction {
            North => { current.0 = current.0 - op.steps }
            East => { current.1 = current.1 + op.steps }
            West => { current.1 = current.1 - op.steps }
            South => { current.0 = current.0 + op.steps }
        };
        current
    }).collect();
    (counter, corners)
}

fn calculate_shoelace_area(p0: &Vec<(i32, i32)>) -> i64 {
    (0..p0.len() - 1)
        .map(|i| {
            let (x0, y0) = p0[i];
            let (x1, y1) = p0[i + 1];
            x0 as i64 * y1 as i64 - x1 as i64 * y0 as i64
        }).sum::<i64>().abs() / 2
}

#[cfg(test)]
mod tests {
    use crate::{solve};

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input, false);
        assert_eq!(result, 62);
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result, 952408144115);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
    };
        input
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input18");
        let part1 = solve(input, false);
        let part2 = solve(input, true);
        assert_eq!(part1, 46359);
        assert_eq!(part2, 59574883048274);
    }
}
