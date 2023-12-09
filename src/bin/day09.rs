
fn main() {
    let input = include_str!("./inputs/input09");
    let (part1, part2) = solve(input);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn solve(p0: &str) -> (i32, i32) {
    let mut sequel_guesses: Vec<i32> = vec![];
    let mut prequel_guesses: Vec<i32> = vec![];
    for history in p0.lines() {
        let mut histories_with_steps: Vec<Vec<i32>> = vec![];
        let mut values: Vec<i32> = history.split(" ").filter_map(|v| v.parse::<i32>().ok()).collect();
        histories_with_steps.push(values.clone());
        loop {
            let mut diff: Vec<i32> = vec![];
            for (i, value) in values.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                diff.push(value - values[i - 1]);
            }
            histories_with_steps.push(diff.clone());
            if diff.iter().all(|v| *v == 0) {
                break;
            }
            values = diff;
        }
        //histories_with_steps = dbg!(histories_with_steps);

        let mut increase = 0;
        let mut decrease = 0;
        for (i, diffs) in histories_with_steps.iter().rev().enumerate() {
            if i == 0 {
                continue;
            }
            increase = diffs.last().unwrap() + increase;
            decrease = diffs.first().unwrap() - decrease;
        }
        sequel_guesses.push(increase);
        prequel_guesses.push(decrease);
    }
    (sequel_guesses.into_iter().sum(), prequel_guesses.into_iter().sum())
}


#[cfg(test)]
mod tests {
    use crate::{solve};

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input);
        assert_eq!(result.0, 114);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"
    };
        input
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input);
        assert_eq!(result.1, 2);
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input09");
        let part1 = solve(input);
        assert_eq!(part1.0, 2008960228);
        assert_eq!(part1.1, 1097);
    }
}
