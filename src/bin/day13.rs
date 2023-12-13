fn main() {
    let input = include_str!("./inputs/input13");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn solve(p0: &str, part2: bool) -> usize {
    p0.split("\n\n")
        .map(|pattern| {
            pattern.lines().map(|line| line.chars().collect()).collect()
        })
        // iterator of matrices = Vec<Vec<char>>
        .map(|matrix| get_mirror_count(&matrix, part2))
        .sum()
}

fn get_mirror_count(matrix: &Vec<Vec<char>>, part2: bool) -> usize {
    if let Some(row_mirror) = get_mirror(matrix, part2) {
        return (row_mirror) * 100;
    }
    if let Some(column_mirror) = get_mirror(&transpose(matrix), part2) {
        return column_mirror;
    }
    return 0;
}

// fn matrix_printer(matrix: &Vec<Vec<char>>) {
//     for line in matrix.iter() {
//         println!("{}", line.iter().map(|c| format!("{}|", *c)).collect::<String>());
//     }
// }

fn get_mirror(matrix: &Vec<Vec<char>>, part2: bool) -> Option<usize> {
    matrix.iter().enumerate()
        // get candidates => subsequent mirrored lines or subsequent mirrored lines with one smudge for part 2
        .filter(|(i, _line)| *i < (matrix.len() - 1) &&
            // mirrored                     or mirrored with one smudge
            (matrix[*i] == matrix[*i + 1] || (part2 && have_one_difference(&matrix[*i], &matrix[*i + 1]))))
        .map(|(i, _line)| (i, i + 1))
        .filter(|&(c1, c2)| {
            let is_part1_solution = !part2 && is_clean_mirror(matrix, c1, c2);
            let is_part2_solution = part2 && is_smudged_mirror(matrix, c1, c2);
            return is_part1_solution || is_part2_solution;
        }).next().map(|(_c1, c2)| c2)
}

fn is_clean_mirror(matrix: &Vec<Vec<char>>, c1: usize, c2: usize) -> bool {
    for i in 1..=matrix.len() {
        // ran out of stuff to mirror => winner
        if i > c1 || c2 + i >= matrix.len() {
            return true;
        } else if matrix[c1 - i] != matrix[c2 + i] {
            return false;
        }
    }
    false
}

fn is_smudged_mirror(matrix: &Vec<Vec<char>>, c1: usize, c2: usize) -> bool {
    // smudge could be already in the first mirrored lines => no further, allowed
    let mut smudge_counter = if have_one_difference(&matrix[c1], &matrix[c2]) { 1 } else { 0 };
    for i in 1..=matrix.len() {
        // ran out of stuff to mirror and found one smudge => winner
        if (i > c1 || c2 + i >= matrix.len()) && smudge_counter == 1 {
            return true;
        } else if smudge_counter > 1 {
            // already more than two smudges needed to be a mirror, get out
            return false;
        } else if (i > c1 || c2 + i >= matrix.len()) && smudge_counter == 0 {
            // ran out of stuff to mirror and smudge is 0
            return false;
        } else {
            smudge_counter += count_differences(&matrix[c1 - i], &matrix[c2 + i]);
        }
    }
    false
}

fn have_one_difference(p0: &Vec<char>, p1: &Vec<char>) -> bool {
    count_differences(p0, p1) == 1
}

fn count_differences(p0: &Vec<char>, p1: &Vec<char>) -> usize {
    p0.iter().zip(p1.iter()).filter(|&(a, b)| a != b).count()
}

// copied from day11
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
        let result = solve(input, false);
        assert_eq!(result, 405);
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result, 400);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    };
        input
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input13");
        let part1 = solve(input, false);
        let part2 = solve(input, true);
        assert_eq!(part1, 33728);
        assert_eq!(part2, 28235);
    }
}
