#![allow(unused_imports)]
use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, enum_map::Enum, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let h = data.len();
    let w = data[0].len();

    let y: usize = 0;
    let x: usize = 1;

    let goal_y: usize = h - 1;
    let goal_x: usize = w - 2;

    // (y,x,path_length,seen)
    let mut queue = VecDeque::from([(y, x, 0_usize, HashSet::from([(y, x)]))]);
    let mut longest = 0;
    while let Some((y, x, length, seen)) = queue.pop_front() {
        if (y, x) == (goal_y, goal_x) {
            longest = longest.max(length);
            continue;
        }

        if b"^.".contains(&data[y][x])
            && y > 0
            && !seen.contains(&(y - 1, x))
            && data[y - 1][x] != b'#'
        {
            let y = y - 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if b"v.".contains(&data[y][x])
            && y + 1 < h
            && !seen.contains(&(y + 1, x))
            && data[y + 1][x] != b'#'
        {
            let y = y + 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if b"<.".contains(&data[y][x])
            && x > 0
            && !seen.contains(&(y, x - 1))
            && data[y][x - 1] != b'#'
        {
            let x = x - 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if b">.".contains(&data[y][x])
            && x + 1 < w
            && !seen.contains(&(y, x + 1))
            && data[y][x + 1] != b'#'
        {
            let x = x + 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
    }
    longest
}

fn part_2(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let h = data.len();
    let w = data[0].len();

    let y: usize = 0;
    let x: usize = 1;

    let goal_y: usize = h - 1;
    let goal_x: usize = w - 2;

    // (y,x,path_length,seen)
    let mut queue = VecDeque::from([(y, x, 0_usize, HashSet::from([(y, x)]))]);
    let mut longest = 0;
    while let Some((y, x, length, seen)) = queue.pop_front() {
        if (y, x) == (goal_y, goal_x) {
            longest = longest.max(length);
            continue;
        }

        unsafe {
            static mut COUNTER: usize = 0;
            COUNTER += 1;
            if COUNTER % 16384 == 0 {
                eprintln!("{}: {} queue length", COUNTER, queue.len());
            }
        }

        if y > 0 && !seen.contains(&(y - 1, x)) && data[y - 1][x] != b'#' {
            let y = y - 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if y + 1 < h && !seen.contains(&(y + 1, x)) && data[y + 1][x] != b'#' {
            let y = y + 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if x > 0 && !seen.contains(&(y, x - 1)) && data[y][x - 1] != b'#' {
            let x = x - 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if x + 1 < w && !seen.contains(&(y, x + 1)) && data[y][x + 1] != b'#' {
            let x = x + 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
    }
    longest
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:23:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:23:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    assert_eq!(part_1(input), 94);
    assert_eq!(part_2(input), 154);
}
