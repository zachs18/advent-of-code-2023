#![allow(clippy::let_and_return)]
use std::{cmp::Ordering, ops::Range};

use aoc_driver::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Default, Debug)]
struct RangeMapElement {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

#[derive(Default, Debug)]
struct RangeMap {
    // sorted by dst, which are nonoverlapping
    elements: Vec<RangeMapElement>,
}

#[derive(Default, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    // each sorted by dst
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

#[derive(Default, Debug)]
struct AlmanacPart2 {
    seeds: Vec<Range<u64>>,
    // each sorted by src
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

fn parse_range_map(lines: &[&str]) -> RangeMap {
    let parse_element = |line: &str| {
        let mut fields = line
            .split_whitespace()
            .map(|field| field.parse::<u64>().unwrap());
        let dst_start = fields.next().unwrap();
        let src_start = fields.next().unwrap();
        let len = fields.next().unwrap();
        RangeMapElement {
            dst_start,
            src_start,
            len,
        }
    };
    let mut elements = lines.iter().copied().map(parse_element).collect_vec();
    elements.sort_unstable_by_key(|element| element.dst_start);
    RangeMap { elements }
}

fn parse_range_map_part2(lines: &[&str]) -> RangeMap {
    let parse_element = |line: &str| {
        let mut fields = line
            .split_whitespace()
            .map(|field| field.parse::<u64>().unwrap());
        let dst_start = fields.next().unwrap();
        let src_start = fields.next().unwrap();
        let len = fields.next().unwrap();
        RangeMapElement {
            dst_start,
            src_start,
            len,
        }
    };
    let mut elements = lines.iter().copied().map(parse_element).collect_vec();
    elements.sort_unstable_by_key(|element| element.src_start);
    RangeMap { elements }
}

impl RangeMap {
    fn get(&self, dst: u64) -> u64 {
        self.get_inner(dst).unwrap_or(dst)
    }

    fn get_inner(&self, dst: u64) -> Option<u64> {
        let idx = self
            .elements
            .binary_search_by(|element| {
                if element.dst_start > dst {
                    Ordering::Greater
                } else if element.dst_start + element.len <= dst {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .ok()?;
        let element = &self.elements[idx];
        let offset = dst - element.dst_start;
        Some(element.src_start + offset)
    }

    fn get_reverse(&self, src: u64) -> u64 {
        self.get_reverse_inner(src).unwrap_or(src)
    }

    fn get_reverse_inner(&self, src: u64) -> Option<u64> {
        let idx = self
            .elements
            .binary_search_by(|element| {
                if element.src_start > src {
                    Ordering::Greater
                } else if element.src_start + element.len <= src {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .ok()?;
        let element = &self.elements[idx];
        let offset = src - element.src_start;
        Some(element.dst_start + offset)
    }
}

fn parse(input: &str) -> Almanac {
    let lines = aoc_2023::lines(input);

    let mut groups = lines.split(|line| line.is_empty());

    let mut seeds = groups.next().unwrap()[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect_vec();
    seeds.sort_unstable();
    let seed_to_soil = parse_range_map(&groups.next().unwrap()[1..]);
    let soil_to_fertilizer = parse_range_map(&groups.next().unwrap()[1..]);
    let fertilizer_to_water = parse_range_map(&groups.next().unwrap()[1..]);
    let water_to_light = parse_range_map(&groups.next().unwrap()[1..]);
    let light_to_temperature = parse_range_map(&groups.next().unwrap()[1..]);
    let temperature_to_humidity = parse_range_map(&groups.next().unwrap()[1..]);
    let humidity_to_location = parse_range_map(&groups.next().unwrap()[1..]);

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn parse_part2(input: &str) -> AlmanacPart2 {
    let lines = aoc_2023::lines(input);

    let mut groups = lines.split(|line| line.is_empty());

    let mut seeds = groups.next().unwrap()[0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .tuples::<(_, _)>()
        .map(|(start, len)| (start..start + len))
        .collect_vec();
    seeds.sort_unstable_by_key(|range| range.start);
    let seed_to_soil = parse_range_map_part2(&groups.next().unwrap()[1..]);
    let soil_to_fertilizer = parse_range_map_part2(&groups.next().unwrap()[1..]);
    let fertilizer_to_water = parse_range_map_part2(&groups.next().unwrap()[1..]);
    let water_to_light = parse_range_map_part2(&groups.next().unwrap()[1..]);
    let light_to_temperature = parse_range_map_part2(&groups.next().unwrap()[1..]);
    let temperature_to_humidity = parse_range_map_part2(&groups.next().unwrap()[1..]);
    let humidity_to_location = parse_range_map_part2(&groups.next().unwrap()[1..]);

    AlmanacPart2 {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn find_seed(almanac: &Almanac, location: u64) -> u64 {
    let humidity = almanac.humidity_to_location.get(location);
    let temperature = almanac.temperature_to_humidity.get(humidity);
    let light = almanac.light_to_temperature.get(temperature);
    let water = almanac.water_to_light.get(light);
    let fertilizer = almanac.fertilizer_to_water.get(water);
    let soil = almanac.soil_to_fertilizer.get(fertilizer);
    let seed = almanac.seed_to_soil.get(soil);
    seed
}

fn find_location2(almanac: &AlmanacPart2, seed: u64) -> u64 {
    let soil = almanac.seed_to_soil.get_reverse(seed);
    let fertilizer = almanac.soil_to_fertilizer.get_reverse(soil);
    let water = almanac.fertilizer_to_water.get_reverse(fertilizer);
    let light = almanac.water_to_light.get_reverse(water);
    let temperature = almanac.light_to_temperature.get_reverse(light);
    let humidity = almanac.temperature_to_humidity.get_reverse(temperature);
    let location = almanac.humidity_to_location.get_reverse(humidity);

    location
}

fn part_1(input: &str) -> u64 {
    let almanac = parse(input);
    for location in 0.. {
        let seed = find_seed(&almanac, location);
        if almanac.seeds.binary_search(&seed).is_ok() {
            return location;
        }
    }
    todo!()
}

fn part_2(input: &str) -> u64 {
    let almanac = parse_part2(input);
    let seeds = almanac.seeds.par_iter().flat_map(Clone::clone);

    seeds
        .map(|seed| find_location2(&almanac, seed))
        .min()
        .unwrap()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:5:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:5:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "seeds: 79 14 55 13

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
56 93 4";
    // assert_eq!(part_1(input), 35);
    assert_eq!(part_2(input), 46);
}
