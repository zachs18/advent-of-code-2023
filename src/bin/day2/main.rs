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

fn parse(input: &str) -> Vec<Game> {
    input
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
        .collect_vec()
}

fn part_1(#[allow(unused)] games: &Vec<Game>) -> usize {
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
fn part_2(#[allow(unused)] games: &Vec<Game>) -> usize {
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
    let session = session.trim();
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:2:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
    if let Err(error) = aoc_magic!(session, 2023:2:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_1 = both.part_1();
    assert_eq!(part_1(input), &8);
    let part_2 = both.part_2();
    assert_eq!(part_2(input), &2286);
}
