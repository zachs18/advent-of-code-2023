#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

/// TODO: use quadratic instead
fn number_of_ways_to_win(race_time: u64, race_distance: u64) -> usize {
    (0..=race_time)
        .map(|held_time| held_time * (race_time - held_time))
        .filter(|&moved_distance| moved_distance > race_distance)
        .count()
}

fn part_1(input: &str) -> usize {
    let (times, distances) = input.split_once('\n').unwrap();
    let times = times
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|field| field.parse::<u64>().unwrap())
        .collect_vec();
    let distances = distances
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|field| field.parse::<u64>().unwrap())
        .collect_vec();
    times
        .into_iter()
        .zip(distances)
        .map(|(time, dist)| number_of_ways_to_win(time, dist))
        .product()
}

fn part_2(input: &str) -> usize {
    let (time, distance) = input.split_once('\n').unwrap();
    let time = time
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distance
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    number_of_ways_to_win(time, distance)
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:6:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:6:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part_1(input), 288);
    assert_eq!(part_2(input), 71503);
}
