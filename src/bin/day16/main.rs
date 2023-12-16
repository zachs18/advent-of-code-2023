#![allow(unused_imports)]
use std::collections::{BTreeSet, HashMap, HashSet};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn step_beams(
    map: &[&[u8]],
    beams: Vec<(usize, usize, Direction)>,
    seen: &mut HashMap<(usize, usize), BTreeSet<Direction>>,
) -> Vec<(usize, usize, Direction)> {
    let h = map.len();
    let w = map[0].len();
    let mut new_beams = Vec::with_capacity(beams.len() * 2);
    for (y, x, dir) in beams {
        if !seen.entry((y, x)).or_default().insert(dir) {
            // already seen
            continue;
        }
        match (map[y][x], dir) {
            (0_u8..=44_u8, _)
            | (48_u8..=91_u8, _)
            | (93_u8..=123_u8, _)
            | (125_u8..=u8::MAX, _) => unreachable!(),
            // pass-throughs
            (b'.' | b'-', Direction::East) => {
                if x + 1 < w {
                    new_beams.push((y, x + 1, dir));
                }
            }
            (b'.' | b'-', Direction::West) => {
                if x > 0 {
                    new_beams.push((y, x - 1, dir));
                }
            }
            (b'.' | b'|', Direction::South) => {
                if y + 1 < h {
                    new_beams.push((y + 1, x, dir));
                }
            }
            (b'.' | b'|', Direction::North) => {
                if y > 0 {
                    new_beams.push((y - 1, x, dir));
                }
            }
            // mirrors EW
            (b'/', Direction::East) | (b'\\', Direction::West) => {
                if y > 0 {
                    new_beams.push((y - 1, x, Direction::North));
                }
            }
            (b'/', Direction::West) | (b'\\', Direction::East) => {
                if y + 1 < h {
                    new_beams.push((y + 1, x, Direction::South));
                }
            }
            // mirrors NS
            (b'/', Direction::North) | (b'\\', Direction::South) => {
                if x + 1 < w {
                    new_beams.push((y, x + 1, Direction::East));
                }
            }
            (b'/', Direction::South) | (b'\\', Direction::North) => {
                if x > 0 {
                    new_beams.push((y, x - 1, Direction::West));
                }
            }
            // splits
            (b'|', Direction::East | Direction::West) => {
                if y > 0 {
                    new_beams.push((y - 1, x, Direction::North));
                }
                if y + 1 < h {
                    new_beams.push((y + 1, x, Direction::South));
                }
            }
            (b'-', Direction::North | Direction::South) => {
                if x + 1 < w {
                    new_beams.push((y, x + 1, Direction::East));
                }
                if x > 0 {
                    new_beams.push((y, x - 1, Direction::West));
                }
            }
        }
    }
    new_beams
}

fn solve_starting_at(map: &[&[u8]], y: usize, x: usize, dir: Direction) -> usize {
    let mut seen: HashMap<(usize, usize), BTreeSet<Direction>> = HashMap::new();
    let mut beams = vec![(y, x, dir)];

    while !beams.is_empty() {
        beams = step_beams(&map, beams, &mut seen);
    }

    seen.len()
}

fn part_1(input: &str) -> usize {
    let map = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();

    solve_starting_at(&map, 0, 0, Direction::East)
}

fn part_2(input: &str) -> usize {
    let map = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let h = map.len();
    let w = map[0].len();
    (0..h)
        .map(|y| {
            let east_start_count = solve_starting_at(&map, y, 0, Direction::East);
            let west_start_count = solve_starting_at(&map, y, w - 1, Direction::West);
            Ord::max(east_start_count, west_start_count)
        })
        .chain((0..w).map(|x| {
            let south_start_count = solve_starting_at(&map, 0, x, Direction::South);
            let north_start_count = solve_starting_at(&map, h - 1, x, Direction::North);
            Ord::max(south_start_count, north_start_count)
        }))
        .max()
        .unwrap()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:16:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:16:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    assert_eq!(part_1(input), 46);
    assert_eq!(part_2(input), 51);
}
