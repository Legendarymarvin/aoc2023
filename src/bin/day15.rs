use crate::Op::*;

fn main() {
    let input = include_str!("./inputs/input15");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1, part2);
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
enum Op {
    ADD,
    REMOVE,
}

fn solve(p0: &str, part2: bool) -> usize {
    if part2 {
        // this would be cleaner if one would use a crate with an insert-ordered dict.
        let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
        p0.split(",")
            .into_iter()
            .map(|s| if s.contains("=") { (ADD, s.split_once("=").unwrap()) } else { (REMOVE, s.split_once("-").unwrap()) })
            .for_each(|(op, (key, value))| {
                let hash = calc_hash(key);
                match op {
                    ADD => {
                        let i = value.parse::<usize>().unwrap();
                        let vec = boxes.get_mut(hash).unwrap();
                        match vec.iter_mut().find(|(k, _)| *k == key) {
                            Some(entry) => entry.1 = i,
                            None => vec.push((key, i)),
                        };
                    }
                    REMOVE => {
                        if let Some(v) = boxes.get_mut(hash) {
                            v.retain(|(k, _v)| *k != key);
                        }
                    }
                }
            });
        boxes.iter().enumerate().flat_map(|(key, value)| {
            value.iter().enumerate().map(move |(i, (_, v))| {
                //value * box * slot
                v * (key + 1) * (i + 1)
            })
        }).sum()
    } else {
        p0.split(",")
            .into_iter()
            .map(|thing| calc_hash(thing))
            .sum()
    }
}

fn calc_hash(p0: &str) -> usize {
    p0.chars().fold(0, |acc, char| {
        let mut temp_sum = acc + (char as u8 as usize);
        temp_sum *= 17;
        temp_sum % 256
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
        assert_eq!(result, 1320);
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result, 145);
    }

    fn get_small_test() -> &'static str { "HASH" }

    fn get_test_input() -> &'static str {
        let input = indoc! {
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    };
        input
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input15");
        let part1 = solve(input, false);
        let part2 = solve(input, true);
        assert_eq!(part1, 507666);
        assert_eq!(part2, 233537);
    }
}
