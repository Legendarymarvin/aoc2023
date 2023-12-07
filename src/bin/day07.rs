use std::collections::HashMap;
use crate::Hand::{Five, Four, Full, HighCard, OnePair, Three, TwoPair};

#[derive(Debug, Copy, Clone)]
enum Hand {
    Five = 7,
    Four = 6,
    Full = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}


fn main() {
    let input = include_str!("./inputs/input07.txt");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1, part2);
}

fn solve(p0: &str, p2: bool) -> i32 {
    let mut hands_with_bets: Vec<(&str, i32)> = vec![];
    for line in p0.lines() {
        let (hand, bet) = line.split_once(" ").unwrap();
        hands_with_bets.push((hand, bet.parse::<i32>().unwrap()));
    }

    let mut hands_with_values_and_bets = vec![];
    for hand in hands_with_bets.into_iter() {
        let values = determine_values(&hand, p2);
        hands_with_values_and_bets.push((values.0, hand, values.1));
    }

    // sort by the first vec in the tuple, the vec is hand_value and the card values in the correct order => vector sorting gives you correct order
    hands_with_values_and_bets.sort_by(|a, b| a.0.cmp(&b.0));
    dbg!(&hands_with_values_and_bets);

    let mut score = 0;
    for (i, hand) in hands_with_values_and_bets.iter().enumerate() {
        score += (i as i32 + 1) * hand.1.1;
    }
    score
}

fn determine_values(p0: &(&str, i32), p2: bool) -> (Vec<i32>, Hand) {
    // values of hand + cards
    let mut values = vec![];
    let mut single_card_values = vec![];
    // keeping a count per char, mostly for debugging.
    let mut counts: HashMap<char, i32> = HashMap::new();

    for char in p0.0.chars() {
        counts.entry(char)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        single_card_values.push(char.to_digit(10).map(|d| d as i32).unwrap_or_else(|| determine_card_value(char, p2)));
    }

    let joker_count = counts.get(&'J').cloned().unwrap_or(0);
    let hand;
    if p2 {
        let card_counts = counts.clone().iter().filter(|&(k, _)| *k != 'J')
            .map(|(k, v)| (k.clone(), *v))
            .collect::<HashMap<char, i32>>();

        hand = if joker_count == 5 || card_counts.values().any(|v| *v == 5 || (*v + joker_count == 5)) {
            Five
        } else if card_counts.values().any(|v| *v == 4 || (*v + joker_count == 4)) {
            Four
        } else if card_counts.values().count() == 2 && joker_count > 0 || card_counts.values().any(|v| *v == 3) && card_counts.values().any(|v| *v == 2) {
            Full
        } else if card_counts.values().any(|v| *v == 3 || (*v + joker_count == 3)) {
            Three
        } else if card_counts.values().count() == 3 && joker_count > 0 || (joker_count == 0 && card_counts.values().filter(|v| **v == 2).count() == 2){
            TwoPair
        } else if card_counts.values().any(|v| (*v == 2) || (*v + joker_count == 2)) {
            OnePair
        } else {
            HighCard
        };
        values.push(hand.clone() as i32);
    } else {
        hand = if counts.values().any(|v| *v == 5) {
            Five
        } else if counts.values().any(|v| *v == 4) {
            Four
        } else if counts.values().any(|v| *v == 3) && counts.values().any(|v| *v == 2) {
            Full
        } else if counts.values().any(|v| *v == 3) {
            Three
        } else if counts.values().filter(|v| **v == 2).count() == 2 {
            TwoPair
        } else if counts.values().any(|v| *v == 2) {
            OnePair
        } else {
            HighCard
        };
        values.push(hand.clone() as i32);
    }

    values.append(&mut single_card_values);
    // hand as part of result for debugging.
    (values, hand)
}

fn determine_card_value(char: char, p2: bool) -> i32 {
    dbg!(char);
    match char {
        'T' => 10,
        'J' => if p2 {1} else {11},
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!()
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
        assert_eq!(result, 6440);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
    };
        input
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result, 5905);
    }
}
