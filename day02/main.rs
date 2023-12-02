use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let input = include_str!("./input.txt");
    let part1 = solve(input, false);
    println!("{}", part1);
    let part2 = solve(input, true);
    println!("{}", part2);
}

fn solve(p0: &str, part2: bool) -> i32 {
    let max: HashMap<String, i32> = [("red", 12), ("green", 13), ("blue", 14)].iter().cloned().map(|(k,v)| (k.to_string(), v)).collect();
    let mut valid: Vec<String> = vec![];
    let mut powers: Vec<i32> = vec![];

    for line in p0.lines() {
        let start_index = line.find(' ').unwrap() + 1;
        let end_index = line.find(':').unwrap();
        let game_number = line[start_index..end_index].to_string();
        let results = line[end_index + 2..].to_string();

        let mut max_per_game: HashMap<String, i32> = HashMap::new();
        let mut is_valid: bool = true;

        for group in results.split("; ") {
            // this could/should be a String, i32 map, but I assumed there'd be examples like 2 red, 4 blue, 2 red.
            let mut color_map: HashMap<String, Vec<i32>> = HashMap::new();
            for pair in group.split(',') {
                let parts: Vec<&str> = pair.trim().split_whitespace().collect();
                if parts.len() == 2 {
                    let number = parts[0].parse::<i32>().unwrap_or(0);
                    let color = parts[1].to_string();
                    color_map.entry(color).or_insert_with(Vec::new).push(number);
                }
            }

            for (key, values) in color_map {
                if let Some(&max) = max.get(&*key) {
                    let sum: i32 = values.iter().sum();
                    if sum > max {
                        is_valid = false;
                    }
                }
                if !max_per_game.contains_key(&*key) ||
                    (max_per_game.contains_key(&*key) && values.iter().sum::<i32>() > max_per_game.get(&*key).unwrap().abs()) {
                    max_per_game.insert(key.clone(), values.iter().sum::<i32>());
                }
            }
        }

        powers.push(max_per_game.values().product());
        if is_valid {
            valid.push(game_number.clone());
        }
    }
    if part2 {
        powers.iter().sum()
    } else {
        valid.iter().filter_map(|s| s.parse::<i32>().ok()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn part1_might_work() {
        let result = solve("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", false);
        assert_eq!(result, 8)
    }

    #[test]
    fn part2_might_work() {
        let result = solve("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true);
        assert_eq!(result, 2286);
    }
}
