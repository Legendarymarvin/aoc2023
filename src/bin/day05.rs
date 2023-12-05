use std::arch::x86_64::_addcarry_u32;
use std::collections::BTreeMap;
use std::hash::Hash;
use regex::Regex;
use itertools::Itertools;
use crate::Relations::{FertilizerToWater, HumidityToLocation, LightToTemperature, SeedToSoil, SoilToFertilizer, TemperatureToHumidity, WaterToLight};

fn main() {
    let input = include_str!("./inputs/input05.txt");
    let part1 = solve(input, false);
    let part2 = solve(input, true);
    println!("Part1: {}, Part2: {}", part1.0, part2.0);
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Debug)]
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
    let mut seed_values: Vec<i64> = seeds.split(" ").into_iter().map(|v| v.parse::<i64>().unwrap()).collect();
    let mut more_seed_values = vec![];
    if p2 {
        for (i, seed) in seed_values.iter().enumerate() {
            if i % 2 != 0 {
                continue;
            }
            let range = seed_values[i + 1];
            for j in 0..range {
                more_seed_values.push(*seed + j);
            }
        }
        seed_values = more_seed_values.clone();
    }
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
        // for (sources, destination) in maps.get(&Relations::HumidityToLocation).cloned().unwrap().iter()
        //     .sorted_by(|a, b| Ord::cmp(a.1, b.1)) {
        //     for i in destination.0..=destination.1 {
        //         let mut source = i - (destination.0 - sources.0);
        //         let location_candidate = source;
        //             for map in maps.values().rev().skip(1) {
        //             for (start, end) in map.iter() {
        //                 if source >= end.0 && source <= end.1 {
        //                     source = source - (end.0 - start.0);
        //                     break;
        //                 }
        //             }
        //         }
        //         if seed_values.contains(&source) {
        //             return (location_candidate, 42);
        //         }
        //     }
        for i in 0..maps.get(&TemperatureToHumidity).unwrap().values().sorted().last().unwrap().1 {
            let mut source = i;
            for (sources, destination) in maps.get(&Relations::HumidityToLocation).cloned().unwrap().iter()
                .sorted_by(|a, b| Ord::cmp(a.1, b.1)) {
                if source >= destination.0 && source <= destination.1 {
                    source = source - destination.0 - sources.0;
                }
            }
            let location_candidate = source;
            for map in maps.values().rev().skip(1) {
                for (start, end) in map.iter() {
                    if source >= end.0 && source <= end.1 {
                        source = source - (end.0 - start.0);
                        break;
                    }
                }
            }
            if seed_values.contains(&source) {
                return (location_candidate, 42);
            }
        }
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
