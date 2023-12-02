#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Clone)]
struct Game {
    idx: usize,
    sets: Vec<CubeSet>,
}

impl Game {
    fn minimum_possible(&self) -> (usize, usize, usize) {
        let [mut red, mut green, mut blue] = [0; 3];
        for set in &self.sets {
            red = red.max(set.red);
            green = green.max(set.green);
            blue = blue.max(set.blue);
        }
        (red, green, blue)
    }
}

fn part_1(#[allow(unused)] input: &str) -> usize {
    let games = input
        .lines()
        .map(|line| {
            let (game_n, rest) = line.split_once(": ").unwrap();
            let idx: usize = game_n["Game ".len()..].parse().unwrap();
            let sets = rest.split(';').map(|set| {
                let set = set.trim();
                let cubes = set.split(',');
                let [mut red, mut green, mut blue] = [0; 3];
                for cube in cubes {
                    if let Some(count) = cube.strip_suffix(" red") {
                        red = count.trim().parse().unwrap();
                    }
                    if let Some(count) = cube.strip_suffix(" green") {
                        green = count.trim().parse().unwrap();
                    }
                    if let Some(count) = cube.strip_suffix(" blue") {
                        blue = count.trim().parse().unwrap();
                    }
                }
                CubeSet { red, green, blue }
            });
            Game {
                idx,
                sets: sets.collect_vec(),
            }
        })
        .collect_vec();
    let red_total = 12;
    let green_total = 13;
    let blue_total = 14;
    games
        .iter()
        .filter_map(|game| {
            game.sets
                .iter()
                .all(|set| {
                    set.red <= red_total && set.green <= green_total && set.blue <= blue_total
                })
                .then_some(game.idx)
        })
        .sum()
}

fn part_2(#[allow(unused)] input: &str) -> usize {
    let games = input
        .lines()
        .map(|line| {
            let (game_n, rest) = line.split_once(": ").unwrap();
            let idx: usize = game_n["Game ".len()..].parse().unwrap();
            let sets = rest.split(';').map(|set| {
                let set = set.trim();
                let cubes = set.split(',');
                let [mut red, mut green, mut blue] = [0; 3];
                for cube in cubes {
                    if let Some(count) = cube.strip_suffix(" red") {
                        red = count.trim().parse().unwrap();
                    }
                    if let Some(count) = cube.strip_suffix(" green") {
                        green = count.trim().parse().unwrap();
                    }
                    if let Some(count) = cube.strip_suffix(" blue") {
                        blue = count.trim().parse().unwrap();
                    }
                }
                CubeSet { red, green, blue }
            });
            Game {
                idx,
                sets: sets.collect_vec(),
            }
        })
        .collect_vec();
    games
        .iter()
        .map(|game| {
            let (r, g, b) = game.minimum_possible();
            r * g * b
        })
        .sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    if let Err(error) = aoc_magic!(session.trim(), 2023:2:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session.trim(), 2023:2:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}
