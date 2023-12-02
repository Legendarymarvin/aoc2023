use std::collections::HashMap;

fn main() {
    let input = include_str!("./input01.txt");
    let part1 = do_part1(input);
    println!("{}", part1);
    let part2 = do_part2(input);
    println!("{}", part2);
}

fn do_part2(input: &str) -> i32 {
    let digit_words = build_digit_map();
    let mut sum = 0;

    for line in input.lines() {
        let digits = get_digits(line, &digit_words);
        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            let number = first * 10 + last;
            sum += number;
        }
    }

    sum
}

fn get_digits(line: &str, digit_words: &HashMap<&str, i32>) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut current_string = String::new();

    for c in line.chars() {
        for (word, &number) in digit_words {
            if current_string.ends_with(word) {
                digits.push(number);
                // this is an ugly as sin solution, but helps with stuff like oneight and twone
                current_string = current_string.chars().skip(current_string.len()-word.len()+1).collect();
                break;
            }
        }
        if c.is_alphabetic() {
            current_string.push(c);
        } else {
            current_string.clear();
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap() as i32);
            }
        }
    }

    for (word, &number) in digit_words {
        if current_string.ends_with(word) {
            digits.push(number);
            break;
        }
    }

    digits
}


fn build_digit_map() -> HashMap<&'static str, i32> {
    let digit_pairs = [
        ("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6),
        ("seven", 7), ("eight", 8), ("nine", 9),
    ];

    digit_pairs.iter().cloned().collect()
}

fn do_part1(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let digits_in_line: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let first = digits_in_line.first();
        let last = digits_in_line.last();

        match (first, last) {
            (Some(f), Some(l)) => {
                let mut number = String::new();
                number.push(*f);
                number.push(*l);
                match number.parse::<i32>() {
                    Ok(num) => sum += num,
                    Err(e) => println!("Failed to parse number: {}, error: {}", number, e),
                }
            },
            _ => println!("Something somewhere somehow went terribly wrong"),
        }

    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::do_part1;
    use crate::do_part2;
    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    };
        let result = do_part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_might_work() {
        let input = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
    };
        let result = do_part2(input);
        assert_eq!(result, 281);
    }

}
