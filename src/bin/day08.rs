use std::collections::HashMap;
use nom::*;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::alpha1;
use nom::multi::many1;
use nom::sequence::{delimited, tuple};

fn main() {
    let input = include_str!("./inputs/input08.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn part1(p0: &str) -> usize {
    let (instructions, mappings) = parse_input(p0);
    let mut origin = "AAA";
    let mut counter = 0;
    'endless: loop {
        for char in instructions.iter() {
            counter += 1;
            match char {
                'L' => origin = mappings.get(&*origin).unwrap().clone().0,
                'R' => origin = mappings.get(&*origin).unwrap().clone().1,
                _ => panic!("{:?} what are you", char)
            };
            if origin == "ZZZ" {
                break 'endless;
            }
        }
    }
    counter
}

fn part2(p0: &str) -> usize {
    let (instructions, mappings) = parse_input(p0);
    let mut lengths = vec![];
    let no_of_instructions = instructions.len();
    for start in mappings.keys().filter(|k| k.ends_with("A")).cloned().collect::<Vec<&str>>() {
        let mut steps = 0;
        let mut origin = start.clone();

        loop {
            if origin.ends_with("Z") {
                break;
            }

            let direction = instructions[steps % no_of_instructions];
            origin = match direction {
                'L' => mappings.get(&origin).unwrap().0.clone(),
                'R' => mappings.get(&origin).unwrap().1.clone(),
                _ => panic!("Something somewhere somehow went terribly wrong, {}", direction),
            };
            steps += 1;
        }

        lengths.push(steps);
    }

    dbg!(&lengths);
    lengths.into_iter().reduce(lowest_common_multiple).unwrap()
}

fn parse_input(p0: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let first = p0.lines().next().unwrap().trim();
    let (_unprocessed, instructions): (&str, Vec<char>) = many1(alpha1::<_, nom::error::Error<&str>>)(first)
        .map(|(remaining, vec_of_str)| {
            (remaining, vec_of_str.into_iter().flat_map(|s| s.chars()).collect::<Vec<char>>())
        }).unwrap();
    let mut mappings: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in p0.lines().skip(2) {
        let (_ignored, (start, _equal, (left, _comma, right))) = tuple((
            //get the start
            take_while(|c: char| c.is_alphanumeric()),
            // delimiter
            tag(" = "),
            //get inside
            delimited(
                // within ()
                // why do you have to call this method 'char' nom?
                character::complete::char::<&str, nom::error::Error<&str>>('('),
                tuple((
                    take_while(|c: char| c.is_alphanumeric()),
                    tag(", "),
                    take_while(|c: char| c.is_alphanumeric()),
                )),
                character::complete::char::<&str, nom::error::Error<&str>>(')'),
            )
        ))(line).unwrap();
        mappings.insert(start, (left, right));
    }
    dbg!(&instructions.iter().collect::<String>());
    dbg!(&mappings);
    (instructions, mappings)
}

fn lowest_common_multiple(left: usize, right: usize) -> usize {
    left / greatest_common_divisor(left, right) * right
}

fn greatest_common_divisor(left: usize, right: usize) -> usize {
    let mut a = left;
    let mut b = right;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = part1(input);
        assert_eq!(result, 6);
        let input = get_test_input2();
        let result = part1(input);
        assert_eq!(result, 2);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"
    };
        input
    }

    fn get_test_input2() -> &'static str {
        let input = indoc! {"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"};
        input
    }

    fn get_test_input3() -> &'static str {
        let input = indoc! {"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"};
        input
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input3();
        let result = part2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn still_working() {
        let input = include_str!("./inputs/input08.txt");
        let part1 = part1(input);
        let part2 = part2(input);
        assert_eq!(part1, 12737);
        assert_eq!(part2, 9064949303801);
    }
}
