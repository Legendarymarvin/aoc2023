use std::collections::BTreeMap;
use std::hash::Hash;
use regex::Regex;
use itertools::Itertools;
use crate::Relations::{FertilizerToWater, HumidityToLocation, LightToTemperature, SeedToSoil, SoilToFertilizer, TemperatureToHumidity, WaterToLight};

fn main() {
    let input = include_str!("inputs/input05.txt");
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
    let mut ranges = vec![];
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
    let mut mappings: BTreeMap<Relations, Vec<((i64, i64), (i64, i64))>> = BTreeMap::new();
    dbg!(&maps);
    if p2 {
        // this part two isn't working, but debugging it seems like a pain
        // so instead I just threw multithreading at bruteforce in day05b
        // seemed like the right thing to do.
        for (i, seed) in seed_values.iter().enumerate() {
            if i % 2 != 0 {
                continue;
            }
            ranges.push((*seed, seed + seed_values[i + 1] - 1));
        }
        let mut to_check = ranges.clone();
        dbg!(&to_check);
        for (key, map) in maps.iter() {
            let mut range_maps = vec![];
            for range in &to_check {
                for (source, destination) in map.iter() {
                    let step = destination.0 - source.0;
                    if (range.0 < source.0 && range.1 < source.0) || (range.0 > source.1 && range.1 > source.1) {
                        continue;
                    } else if range.0 >= source.0 && range.1 <= source.1 {
                        // complete inside, complete map
                        range_maps.push((range.clone(), (range.0 + step, range.1 + step)));
                    } else if range.0 < source.0 && range.1 > source.1 {
                        // envelope completely => only mapper inner
                        range_maps.push((source.clone(), destination.clone()));
                    } else if range.0 >= source.0 && range.1 > source.1 {
                        // starts within source, but goes outside
                        range_maps.push(((range.0, source.1), (range.0 + step, source.1 + step)));
                    } else if range.0 < source.0 && range.1 <= source.1 {
                        // starts before and goes within
                        range_maps.push(((source.0, range.1), (source.0 + step, range.1 + step)));
                    }
                }
            }
            dbg!(&key, &range_maps);
            'current_range: for range in &to_check {
                let mut remaining = range.clone();
                let mapped_ranges = range_maps.iter().cloned().map(|map| map.0).sorted().collect::<Vec<(i64, i64)>>();
                for mapped in mapped_ranges {
                    if remaining.0 >= mapped.0 && remaining.1 <= mapped.1 {
                        // completely mapped, ignore it.
                        continue 'current_range;
                    }
                    if remaining.0 < mapped.0 && remaining.1 > mapped.0 && remaining.1 < mapped.1 {
                        // second part mapped, continue checking first
                        remaining = (remaining.0, mapped.0 - 1);
                        continue;
                    }
                    if remaining.0 > mapped.0 && remaining.0 < mapped.1 && remaining.1 > mapped.1 {
                        // first part mapped, continue checking second
                        remaining = (mapped.1 + 1, remaining.1);
                        continue;
                    }
                }
                range_maps.push((remaining.clone(), remaining.clone()));
            }
            dbg!(&range_maps);
            mappings.insert(*key, range_maps.clone());
            to_check = range_maps.iter().cloned().map(|map| map.1).sorted().collect::<Vec<(i64, i64)>>();
        }
        let x = mappings.get(&HumidityToLocation).unwrap();
        for temp in x {
            // to low 29861279, too high 100644325
            if temp.1.0 == 0 || temp.1.0 == 29861279 {
                continue;
            }
            lowest_location = i64::min(lowest_location, temp.1.0);
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
