use std::collections::BTreeMap;
use rayon::prelude::*;
use std::hash::Hash;
use regex::Regex;
use itertools::Itertools;
use crate::Relations::{FertilizerToWater, HumidityToLocation, LightToTemperature, SeedToSoil, SoilToFertilizer, TemperatureToHumidity, WaterToLight};

fn main() {
    let input = include_str!("./inputs/fuckyou.txt");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1.0, part2.0);
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Debug, Clone, Copy)]
enum Relations {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}


fn solve(p0: &str, p2: bool) -> (i64, i64) {
    let mut lines = p0.lines();
    let (_, seeds) = lines.next().unwrap_or("").split_once(": ").unwrap();
    let seed_values: Vec<i64> = seeds.split(" ").into_iter().map(|v| v.parse::<i64>().unwrap()).collect();
    let mappings: BTreeMap<Relations, &str> = BTreeMap::from([(SeedToSoil, "seed-to-soil"), (SoilToFertilizer, "soil-to-fertilizer"),
        (FertilizerToWater, "fertilizer-to-water"), (WaterToLight, "water-to-light"), (LightToTemperature, "light-to-temperature"),
        (TemperatureToHumidity, "temperature-to-humidity"), (HumidityToLocation, "humidity-to-location")]);
    let mut maps: BTreeMap<Relations, BTreeMap<(i64, i64), (i64, i64)>> = BTreeMap::from([
        (Relations::SeedToSoil, BTreeMap::new()),
        (Relations::SoilToFertilizer, BTreeMap::new()),
        (Relations::FertilizerToWater, BTreeMap::new()),
        (Relations::WaterToLight, BTreeMap::new()),
        (Relations::LightToTemperature, BTreeMap::new()),
        (Relations::TemperatureToHumidity, BTreeMap::new()),
        (Relations::HumidityToLocation, BTreeMap::new()),
    ]);

    let mut current_map = &Relations::SeedToSoil;
    for line in p0.lines().skip(1) {
        if line.trim().len() <= 1 {
            continue;
        }
        for (k, v) in &mappings {
            if line.starts_with(v) {
                current_map = k;
                continue;
            }
        }

        if Regex::new(r"^[0-9]").unwrap().is_match(line) {
            let (start_destination, start_source, step) = line.split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect_tuple().unwrap();
            maps.get_mut(current_map).unwrap().insert((start_source, start_source + step - 1), (start_destination, start_destination + step - 1));
        }
    }

    let mut lowest_location = i64::MAX;
    if p2 {
        let mut ranges = vec![];
        for (i, seed) in seed_values.iter().enumerate() {
            if i % 2 != 0 {
                continue;
            }
            let range = seed_values[i + 1];
            ranges.push((*seed, *seed + range - 1));
        }
        dbg!(&ranges);
        // brute force best force, took 5hrs with 12 cores. Perfectly reasonable.
        lowest_location = (0..i64::MAX).into_par_iter().find_first(|&i|  {
            let mut value = i;
            dbg!(value);
            'map: for (key, map) in maps.iter().rev() {
                for (source_range, destination_range) in map.iter() {
                    if is_in_range(value, destination_range) {
                        let step = destination_range.0 - source_range.0;
                        value = value - step;
                        continue 'map;
                    }
                }
            }
            for range in &ranges {
                if is_in_range(value, range) {
                    return true;
                }
            }
            return false;
        }).unwrap();
    } else {
        for seed in seed_values {
            let mut current = seed;
            for (_rel, k) in &maps {
                for (start_range, destination_range) in k {
                    if current >= start_range.0 && current <= start_range.1 {
                        current = current + (destination_range.0 - start_range.0);
                        break;
                    }
                }
            }
            lowest_location = i64::min(lowest_location, current);
        }
    }

    (lowest_location, 42)
}

fn is_in_range(p0: i64, p1: &(i64, i64)) -> bool {
    p0 >= p1.0 && p0 <= p1.1
}

#[cfg(test)]
mod tests {
    use crate::solve;

    use indoc::indoc;

    #[test]
    fn part1_might_work() {
        let input = get_test_input();
        let result = solve(input, false);
        assert_eq!(result.0, 35);
    }

    fn get_test_input() -> &'static str {
        let input = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4"
    };
        input
    }

    #[test]
    fn part2_might_work() {
        let input = get_test_input();
        let result = solve(input, true);
        assert_eq!(result.0, 46);
    }
}
