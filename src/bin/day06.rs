use regex::Regex;

fn main() {
    let input = include_str!("./inputs/input06.txt");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn solve(p0: &str, p2: bool) -> usize {
    let mut lines = p0.lines();
    let times: Vec<i64> = parse_line(p2, lines.next().unwrap());
    let distances: Vec<i64> = parse_line(p2, lines.next().unwrap());
    let mut holds_by_race = vec![];

    for (i, time) in times.iter().enumerate() {
        // thought we might need the holds itself, since we don't, a counter is enough.
        // let mut valid_holds = vec![];
        let mut counter = 0;
        let mut been_faster_once = false;
        for hold in 1..=*time {
            let remain = *time - hold;
            // pointless, just for clarity
            let speed = hold;
            if speed * remain > distances[i] {
                been_faster_once = true;
                counter += 1;
                //valid_holds.push(hold);
            } else if been_faster_once {
                // once you been faster and stopped, you'll never be faster again, since it's a parabola.
                // not much of an optimization, but hey.
                break;
            }
        }
        holds_by_race.push(counter);
    }
    holds_by_race.iter().product()
}

fn parse_line(p2: bool, times_line: &str) -> Vec<i64> {
    if !p2 {
        Regex::new(r"\d+").unwrap().find_iter(times_line)
            .filter_map(|num| num.as_str().parse::<i64>().ok())
            .collect()
    } else {
        let big_one = Regex::new(r"\d+").unwrap().find_iter(times_line)
            .map(|num| num.as_str())
            .collect::<String>();
        vec![big_one.parse::<i64>().unwrap()]
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input, false);
        assert_eq!(result, 288);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
"
    };
        input
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result, 71503);
    }
}
