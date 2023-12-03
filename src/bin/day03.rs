use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input03.txt");
    solve(input);
}

fn solve(p0: &str) -> () {
    let lines = p0.lines();
    let matrix: Vec<Vec<char>> = lines.clone().map(|line| line.chars().collect()).collect();
    let mut parts = vec![];
    let mut next_to_star: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for (line_number, line) in lines.clone().enumerate() {
        let mut on_a_number = false;
        let mut current_number = String::new();
        for (char_index, char) in line.chars().enumerate() {
            if !on_a_number && char.is_numeric() {
                current_number = String::from(char);
                on_a_number = true;
            } else if on_a_number && char.is_numeric() {
                current_number.push(char);
            } else if on_a_number && !char.is_numeric() {
                let start_char = if char_index == 0 || char_index == 1 {
                    0
                } else {
                    char_index - current_number.len()
                };
                let start = (line_number, start_char);
                let end = (line_number, char_index - 1);
                if is_engine(start, end, &matrix, &mut next_to_star, current_number.clone()) {
                    parts.push(current_number.parse::<i32>().unwrap());
                }
                on_a_number = false;
                current_number.clear();
            }
        }
        if on_a_number {
            let start_char = line.len() - 1 - current_number.len();
            let start = (line_number, start_char);
            let end = (line_number, line.len() - 1);
            if is_engine(start, end, &matrix, &mut next_to_star, current_number.clone()) {
                parts.push(current_number.parse::<i32>().unwrap());
            }
            current_number.clear();
        }
    }

    println!("Part 1: {}", parts.iter().sum::<i32>());
    let part2: i32 = next_to_star.values()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.iter().product::<i32>())
        .sum();
    println!("Part 2: {}", part2);
}

fn is_engine(start: (usize, usize), end: (usize, usize), matrix: &Vec<Vec<char>>, x: &mut HashMap<(usize, usize), Vec<i32>>, current_number: String) -> bool {
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
                is_engine = true;
            }
            if char == '*' {
                if x.contains_key(&(line_index, char_index)) {
                    let mut values = x.get(&(line_index, char_index)).unwrap().clone();
                    values.push(current_number.parse::<i32>().unwrap());
                    x.insert((line_index, char_index), values);
                } else {
                    x.insert((line_index, char_index), vec![current_number.parse::<i32>().unwrap()]);
                }
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
        let result = solve(input, true);
        assert_eq!(result, 4361);
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
        let result = solve(input, false);
        assert_eq!(result, 467835);
    }
}
