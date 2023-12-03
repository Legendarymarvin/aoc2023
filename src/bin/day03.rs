use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input03.txt");
    let both = solve(input);
    dbg!(both);
}

fn solve(p0: &str) -> (i32, i32) {
    let lines = p0.lines();
    let matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut parts = vec![];
    let mut next_to_star: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (line_number, line) in matrix.iter().enumerate() {
        let mut on_a_number = false;
        let mut current_number = String::new();

        for (char_index, char) in line.iter().enumerate() {
            if !on_a_number && char.is_numeric() {
                current_number = String::from(*char);
                on_a_number = true;
            } else if is_end_of_number(line.len() - 1, on_a_number, char_index, *char) {
                if char.is_numeric() {
                    current_number.push(*char);
                }
                let start_char = get_start_char_index(&mut current_number, char_index);
                let start = (line_number, start_char);
                let end = (line_number, char_index - 1);

                if is_engine(start, end, &matrix, &mut next_to_star, &current_number) {
                    parts.push(current_number.parse::<i32>().unwrap());
                }

                on_a_number = false;
                current_number.clear();
            } else if on_a_number && char.is_numeric() {
                current_number.push(*char);
            }
        }
    }

    let part1 = parts.iter().sum::<i32>();
    let part2 = next_to_star.values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().product::<i32>())
        .sum::<i32>();
    (part1, part2)
}

fn get_start_char_index(current_number: &mut String, char_index: usize) -> usize {
    let start_char = if char_index == 0 || char_index == 1 {
        0
    } else {
        char_index - current_number.len()
    };
    start_char
}

fn is_end_of_number(line_end: usize, on_a_number: bool, char_index: usize, char: char) -> bool {
    (on_a_number && !char.is_numeric())
        || (on_a_number && (char_index == line_end))
}

fn is_engine(start: (usize, usize), end: (usize, usize), matrix: &Vec<Vec<char>>, x: &mut HashMap<(usize, usize), Vec<i32>>, current_number: &String) -> bool {
    let start_line = cmp::max(0, start.0.saturating_sub(1));
    let end_line = cmp::min(end.0 + 1, matrix.len() - 1);

    let mut is_engine = false;

    for line_index in start_line..=end_line {
        let line = &matrix[line_index];
        let start_char = cmp::max(0, start.1.saturating_sub(1));
        let end_char = cmp::min(end.1 + 1, line.len() - 1);

        for char_index in start_char..=end_char {
            let char = line[char_index];
            if !char.is_numeric() && char != '.' {
                // for part 1, you can directly return true here.
                is_engine = true;
            }
            // part 2
            if char == '*' {
                let value = current_number.parse::<i32>().unwrap();
                match x.entry((line_index, char_index)) {
                    Entry::Vacant(e) => { e.insert(vec![value]);},
                    Entry::Occupied(mut e) => { e.get_mut().push(value)}
                };
            }
        }
    }

    is_engine
}

#[cfg(test)]
mod tests {
    use crate::solve;

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
    };
        let result = solve(input);
        assert_eq!(result.0, 4361);
    }

    #[test]
    fn part2_might_work() {
        let input = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
    };
        let result = solve(input);
        assert_eq!(result.1, 467835);
    }
}
