use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("./inputs/input11");
    let (part1, part2) = solve(input, 1000000);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn solve(p0: &str, expansion: usize) -> (usize, usize) {
    let galaxy = read_and_expand_map(p0);
    let stars: Vec<(usize, usize)> = map_stars(&galaxy);
    let mut distances: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    for star in stars.clone().iter().cloned() {
        for other_star in stars.iter().cloned() {
            if star == other_star {
                continue;
            }
            if distances.contains_key(&(star, other_star)) || distances.contains_key(&(other_star, star)) {
                continue
            }
            let y_distance = usize::max(star.0, other_star.0) - usize::min(star.0, other_star.0);
            let x_distance = usize::max(star.1, other_star.1) - usize::min(star.1, other_star.1);
            distances.insert((star, other_star), y_distance + x_distance);
        }
    }

    let (empty_rows, empty_columns, galaxy) = ignore_empty_rows(p0);
    let stars: Vec<(usize, usize)> = map_stars(&galaxy);

    let mut big_distances: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    dbg!(galaxy);
    dbg!(&stars);
    for star in stars.clone().iter().cloned() {
        for other_star in stars.iter().cloned() {
            if star == other_star {
                continue;
            }
            if big_distances.contains_key(&(star, other_star)) || big_distances.contains_key(&(other_star, star)) {
                continue
            }
            dbg!(star, other_star);
            let big_y = usize::max(star.0, other_star.0);
            let small_y = usize::min(star.0, other_star.0);
            let big_x = usize::max(star.1, other_star.1);
            let small_x = usize::min(star.1, other_star.1);
            let mut y_distance = big_y - small_y;
            let mut x_distance = big_x - small_x;

            for i in small_y..big_y {
                if empty_rows.contains(&i) {
                    y_distance = y_distance + expansion - 1;
                }
            }

            for i in small_x..big_x {
                if empty_columns.contains(&i) {
                    x_distance = x_distance + expansion - 1;
                }
            }

            dbg!(y_distance, x_distance);
            big_distances.insert((star, other_star), y_distance + x_distance);
        }
    }


    (distances.values().sum(), big_distances.values().sum())
}
fn ignore_empty_rows(p0: &str) -> (Vec<usize>, Vec<usize>, Vec<Vec<char>>) {
    let mut empty_rows: Vec<usize> = vec![];
    // as everyone knows, galaxies are quadratic
    let mut empty_columns = vec![];
    let mut gal:Vec<Vec<char>> = vec![];


    for (y, line) in p0.lines().enumerate() {
        let mut row = vec![];
        for (x, char) in line.chars().enumerate() {
            row.push(char);
        }
        gal.push(row);
    }

    let mut transposed = transpose(dbg!(gal));
    for (i, line) in transposed.iter().enumerate() {
        if line.iter().all(|c| *c == '.') {
            empty_columns.push(i);
        }
    }
    let mut actual = transpose(dbg!(transposed));
    for (i, line) in actual.iter().enumerate() {
        if line.iter().all(|c| *c == '.') {
            empty_rows.push(i);
        }
    }
    (empty_rows, empty_columns, actual)
}

fn map_stars(p0: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut stars = vec![];
    for (y, row) in p0.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == '#' {
                stars.push((y, x));
            }
        }
    }
    stars
}

fn read_and_expand_map(p0: &str) -> Vec<Vec<char>> {


    let mut gal:Vec<Vec<char>> = vec![];


    for (y, line) in p0.lines().enumerate() {
        let mut row = vec![];
        for (x, char) in line.chars().enumerate() {
            row.push(char);
        }
        gal.push(row);
    }

    let mut transposed = transpose(dbg!(gal));
    let mut expanded = vec![];
    for line in transposed {
        expanded.push(line.clone());
        if line.iter().all(|c| *c == '.') {
            expanded.push(line.clone());
        }
    }
    let mut actual = transpose(dbg!(expanded));
    let mut expanded = vec![];
    for line in actual {
        expanded.push(line.clone());
        if line.iter().all(|c| *c == '.') {
            expanded.push(line.clone());
        }
    }
    dbg!(expanded)
}

fn transpose(p0: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed = vec![];
    for (j, row) in p0.iter().enumerate() {
        for (i, char) in row.iter().enumerate() {
            if (j == 0) {
                transposed.push(vec![]);
            }
            transposed[i].push(*char);
        }
    }
    transposed
}

#[cfg(test)]
mod tests {
    use crate::{solve};

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input, 42);
        assert_eq!(result.0, 374);
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, 10);
        assert_eq!(result.1, 1030);
        let result = solve(input, 100);
        assert_eq!(result.1, 8410);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    };
        input
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input10");
        let part1 = solve(input, 1000000);
        assert_eq!(part1.0, 6838);
        assert_eq!(part1.1, 451);
    }

}
