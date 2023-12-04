use std::collections::HashMap;

fn main() {
    let input = include_str!("./inputs/input04.txt");
    let both = solve(input);
    println!("Part1: {}, Part2: {}", both.0, both.1);
}

fn solve(p0: &str) -> (i32, i32) {
    let mut score: i32 = 0;
    let mut card_counts: HashMap<usize, i32> = HashMap::new();
    for c in 1..=p0.lines().count() {
        card_counts.insert(c, 1);
    }

    for (i, line) in p0.lines().enumerate() {
        if line.len() <= 0 {
            continue;
        }
        let winner_and_picked = line
            .split(": ")
            .skip(1).next().unwrap()
            .splitn(2, " | ")
            .map(str::trim).collect::<Vec<&str>>();

        let winning_numbers = split_and_parse(&winner_and_picked[0]);
        let picked = split_and_parse(&winner_and_picked[1]);
        let winners: Vec<i32> = picked.iter()
            .filter(|v| winning_numbers.contains(v)).cloned().collect();

        // either catch no winners here or catch not adding 2^0 to the score later
        if winners.len() > 0 {
            // part 1
            score += 2i32.pow((winners.len().saturating_sub(1)) as u32);

            // part 2
            let current_card_count = card_counts.get(&(i+1)).unwrap_or(&1).clone();
            for j in 1..=winners.len() {
                let won_card = i + 1 + j;
                card_counts.entry(won_card)
                    .and_modify(|e| *e += current_card_count)
                    .or_insert(1 + current_card_count);
            }
        }

    }
    (score, card_counts.values().sum())
}

fn split_and_parse(numbers: &&str) -> Vec<i32> {
    numbers.split(" ").map(|v| v.parse::<i32>()).filter_map(|v| v.ok()).collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input);
        assert_eq!(result.0, 13);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    };
        input
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input);
        assert_eq!(result.1, 30);
    }
}
