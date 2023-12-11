use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("./inputs/input11");
    let (part1, part2) = solve(input, 1000000);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn solve(p0: &str, expansion: usize) -> (usize, usize) {
    let (empty_rows, empty_columns, galaxy) = read_map(p0);
    let stars: Vec<(usize, usize)> = map_stars(&galaxy);

    let mut distances: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    fill_distances(2, &empty_rows, &empty_columns, &stars, &mut distances);
    let part1 = distances.values().sum();

    distances.clear();
    fill_distances(expansion, &empty_rows, &empty_columns, &stars, &mut distances);
    let part2 = distances.values().sum();


    (part1, part2)
}

fn fill_distances(expansion: usize, empty_rows: &Vec<usize>, empty_columns: &Vec<usize>, stars: &Vec<(usize, usize)>, big_distances: &mut HashMap<((usize, usize), (usize, usize)), usize>) {
    stars.iter()
        .flat_map(|&star| {
            stars.iter()
                .filter_map(move |&other_star| if other_star != star { Some((star, other_star)) } else { None })
        }).for_each(|(star, other_star)| {
        if star == other_star || big_distances.contains_key(&(star, other_star)) || big_distances.contains_key(&(other_star, star)) {
            return;
        }

        let (small_y, big_y) = vec![star.0, other_star.0].iter().cloned().sorted().collect_tuple().unwrap();
        let (small_x, big_x) = vec![star.1, other_star.1].iter().cloned().sorted().collect_tuple().unwrap();

        let y_distance = (big_y - small_y)
            + (small_y..big_y).into_iter()
            .filter(|i| empty_rows.contains(&i))
            .count() * (expansion - 1);

        let x_distance = (big_x - small_x)
            + (small_x..big_x).into_iter()
            .filter(|i| empty_columns.contains(&i))
            .count() * (expansion - 1);

        big_distances.insert((star, other_star), y_distance + x_distance);
    });
}

fn read_map(p0: &str) -> (Vec<usize>, Vec<usize>, Vec<Vec<char>>) {
    let gal: Vec<Vec<char>> = p0.lines()
        .map(|line| line.chars().collect())
        .collect();

    let empty_columns = transpose(&gal)
        .into_iter()
        .enumerate()
        .filter_map(|(i, line)| if line.iter().all(|c| *c == '.') { Some(i) } else { None })
        .collect();

    let empty_rows = gal.iter().enumerate()
        .filter_map(|(i, line)| if line.iter().all(|c| *c == '.') { Some(i) } else { None })
        .collect();

    (empty_rows, empty_columns, gal)
}

fn map_stars(p0: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    p0.iter().enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate()
                .filter_map(move |(x, c)| if *c == '#' { Some((y, x)) } else { None })
        }).collect()
}

fn transpose(p0: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    p0.iter()
        .fold(vec![Vec::new(); p0[0].len()], |mut acc, row| {
            row.iter().enumerate().for_each(|(i, c)| acc[i].push(*c));
            acc
        })
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
        let input = include_str!("./inputs/input11");
        let part1 = solve(input, 1000000);
        assert_eq!(part1.0, 10165598);
        assert_eq!(part1.1, 678728808158);
    }
}
