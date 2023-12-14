use std::collections::BTreeMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("./inputs/input14");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1, part2);
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
enum Thing {
    Rock,
    Fixed,
    Empty,
}

fn solve(p0: &str, part2: bool) -> usize {
    let mut matrix: BTreeMap<(usize, usize), Thing> = BTreeMap::new();
    let length = p0.lines().next().unwrap().len();
    p0.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, thing)| {
            match thing {
                '#' => matrix.insert((y, x), Thing::Fixed),
                'O' => matrix.insert((y, x), Thing::Rock),
                '.' => matrix.insert((y, x), Thing::Empty),
                _ => { panic!("WHAT IS THIS! {}", &thing) }
            };
        })
    });

    if part2 {
        let mut results: Vec<Vec<(usize, usize)>> = vec![];
        let mut cycled = matrix.clone();

        for _ in 0..1000 {
            // let's do a thousand and hope to find a pattern;
            cycled = cycle(&cycled, length);
            results.push(get_stones(&cycled));
        }
        let pattern = please_find_a_cycle(&results).unwrap();
        let pattern_length = pattern.len();
        let dist = 1_000_000_000 - 1000;
        let pattern_pos = if dist % pattern_length == 0 {pattern_length-1} else {(dist % pattern_length) - 1};
        let candidate = pattern[pattern_pos].clone();

        candidate.iter().map(|rock_pos| length - rock_pos.0).sum()
    } else {
        //print_matrix(&matrix, length);
        calc_weight(&mut tilt_north(matrix, length), length)
    }
}

fn calc_weight(matrix: &mut BTreeMap<(usize, usize), Thing>, length: usize) -> usize {
    let mut sum = 0;
    for y in 0..length {
        for x in 0..length {
            if matrix.get(&(y, x)).unwrap() == &Thing::Rock {
                sum += length - y;
            }
        }
    }
    sum
}

fn please_find_a_cycle(rock_lists: &Vec<Vec<(usize, usize)>>) -> Option<Vec<Vec<(usize, usize)>>> {
    // three repetitions demanded + 1 because of distance
    for size in 1..=rock_lists.len()/4 {
        for dist in 0..size {
            let mut slices = vec![];
            for i in 0..3 {
                let end = rock_lists.len() - dist - i * size;
                let start = rock_lists.len() - dist - (i+1) * size;
                slices.push(&rock_lists[start..end]);
            }
            if slices.iter().all_equal() {
                return Some(slices[0].iter().cloned().collect())
            }
        }
    }
    None
}

fn get_stones(p0: &BTreeMap<(usize, usize), Thing>) -> Vec<(usize, usize)> {
    p0.iter().filter(|(_k, value)| **value == Thing::Rock).map(|(key, _v)| *key).collect()
}

fn cycle(p0: &BTreeMap<(usize, usize), Thing>, length: usize) -> BTreeMap<(usize, usize), Thing> {
    let mut after = p0.clone();
    // I started generalizing the tilt methods, but the results were ugly, I'll live with the code duplication.
    after = tilt_north(after, length);
    after = tilt_west(after, length);
    after = tilt_south(after, length);
    after = tilt_east(after, length);
    after
}

fn tilt_north(p0: BTreeMap<(usize, usize), Thing>, length: usize) -> BTreeMap<(usize, usize), Thing> {
    let mut after = p0.clone();
    for y in 0..length {
        for x in 0..length {
            if y == 0 {
                continue;
            }
            if after.get(&(y, x)).unwrap() != &Thing::Rock {
                continue;
            }

            'tilting: for tilt in 1..=y {
                let current = y - tilt + 1;
                let tilted = y - tilt;
                if after.get(&(tilted, x)).unwrap() == &Thing::Empty {
                    after.insert((tilted, x), Thing::Rock);
                    after.insert((current, x), Thing::Empty);
                } else {
                    break 'tilting;
                }
            }
        }
    }
    after
}

fn tilt_south(p0: BTreeMap<(usize, usize), Thing>, length: usize) -> BTreeMap<(usize, usize), Thing> {
    let mut after = p0.clone();
    for y in (0..length).rev() {
        for x in 0..length {
            if y == length - 1 {
                continue;
            }
            if after.get(&(y, x)).unwrap() != &Thing::Rock {
                continue;
            }

            'tilting: for tilt in 1..(length - y) {
                let current = y + tilt - 1;
                let tilted = y + tilt;
                if after.get(&(tilted, x)).unwrap() == &Thing::Empty {
                    after.insert((tilted, x), Thing::Rock);
                    after.insert((current, x), Thing::Empty);
                } else {
                    break 'tilting;
                }
            }
        }
    }
    after
}



fn tilt_east(p0: BTreeMap<(usize, usize), Thing>, length: usize) -> BTreeMap<(usize, usize), Thing> {
    let mut after = p0.clone();
    for x in (0..length).rev() {
        for y in 0..length {
            if x == length - 1 {
                continue;
            }
            if after.get(&(y, x)).unwrap() != &Thing::Rock {
                continue;
            }

            'tilting: for tilt in 1..(length - x) {
                let current = x + tilt - 1;
                let tilted = x + tilt;
                if after.get(&(y, tilted)).unwrap() == &Thing::Empty {
                    after.insert((y, tilted), Thing::Rock);
                    after.insert((y, current), Thing::Empty);
                } else {
                    break 'tilting;
                }
            }
        }
    }
    after
}
fn tilt_west(p0: BTreeMap<(usize, usize), Thing>, length: usize) -> BTreeMap<(usize, usize), Thing> {
    let mut after = p0.clone();
    for x in 0..length {
        for y in 0..length {
            if x == 0 {
                continue;
            }
            if after.get(&(y, x)).unwrap() != &Thing::Rock {
                continue;
            }

            'tilting: for tilt in 1..=x {
                let current = x - tilt + 1;
                let tilted = x - tilt;
                if after.get(&(y, tilted)).unwrap() == &Thing::Empty {
                    after.insert((y, tilted), Thing::Rock);
                    after.insert((y, current), Thing::Empty);
                } else {
                    break 'tilting;
                }
            }
        }
    }
    after
}

#[allow(dead_code)]
fn print_matrix(p0: &BTreeMap<(usize, usize), Thing>, length: usize) {
    for y in 0..length {
        let mut row = String::new();
        for x in 0..length {
            let thing: &Thing = p0.get(&(y, x)).unwrap();
            row.push(match *thing {
                Thing::Rock => 'O',
                Thing::Empty => '.',
                Thing::Fixed => '#',
            })
        }
        println!("{}", row);
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve};

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input, false);
        assert_eq!(result, 136);
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result, 64);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    };
        input
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input14");
        let part1 = solve(input, false);
        let part2 = solve(input, true);
        assert_eq!(part1, 113424);
        assert_eq!(part2, 96003);
    }
}
