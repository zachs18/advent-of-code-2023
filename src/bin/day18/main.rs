#![allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use aoc_2023::*;
use aoc_driver::*;
use derive_more::{Add, From, Into, Mul};
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, From, Into, Add, Mul)]
struct Point {
    y: isize,
    x: isize,
}

fn signed_area_clockwise_from_origin_in_halves(p2: Point, p3: Point) -> isize {
    let Point { y: y1, x: x1 } = (0, 0).into();
    let Point { y: y2, x: x2 } = p2;
    let Point { y: y3, x: x3 } = p3;
    let a = x1 * (y2 - y3);
    let b = x2 * (y3 - y1);
    let c = x3 * (y1 - y2);
    a + b + c
}

impl Direction {
    fn offset(self) -> Point {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
        .into()
    }
}

/// Triangle form of the shoelace theorem.
fn calculate_area(path: &[(Direction, usize)]) -> usize {
    let mut total_side_length: usize = 0;
    let mut current_signed_area_in_halves: isize = 0;
    let mut prev_point: Point = (0, 0).into();
    for &(direction, dist) in path {
        total_side_length += dist;
        let current_point = prev_point + direction.offset() * dist as isize;
        current_signed_area_in_halves +=
            signed_area_clockwise_from_origin_in_halves(prev_point, current_point);
        prev_point = current_point;
    }

    current_signed_area_in_halves.abs_diff(0) / 2 + total_side_length / 2 + 1
}

fn solve(input: &str) -> (usize, usize) {
    let (path1, path2): (Vec<_>, Vec<_>) = input
        .lines()
        .map(str::trim)
        .map(|line| {
            let mut fields = line.split_whitespace();
            let direction1 = match fields.next().unwrap() {
                "R" => Direction::East,
                "U" => Direction::North,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => unreachable!(),
            };
            let count1 = fields.next().unwrap().parse::<usize>().unwrap();
            let rgb = fields.next().unwrap();
            dbg!(&rgb[2..rgb.len() - 2]);
            let count2 = usize::from_str_radix(&rgb[2..rgb.len() - 2], 16).unwrap();
            let direction2 = match &rgb[rgb.len() - 2..][..1] {
                "0" => Direction::East,
                "3" => Direction::North,
                "1" => Direction::South,
                "2" => Direction::West,
                _ => unreachable!(),
            };

            ((direction1, count1), (direction2, count2))
        })
        .unzip();
    (calculate_area(&path1), calculate_area(&path2))
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let mut both = SingleFunction::new(solve);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:18:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
    if let Err(error) = aoc_magic!(session, 2023:18:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    assert_eq!(solve(input), (62, 952408144115));
}
