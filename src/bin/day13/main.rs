#![allow(unused_imports)]
use std::ops::Deref;

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

fn eq_rev<T: Eq>(s1: impl Iterator<Item = T>, s2: impl DoubleEndedIterator<Item = T>) -> bool {
    s1.eq(s2.rev())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SplitLines {
    v_splits: Vec<usize>,
    h_splits: Vec<usize>,
}

impl SplitLines {
    fn summarize(self) -> Option<usize> {
        match (&self.v_splits[..], &self.h_splits[..]) {
            ([], &[y]) => Some(100 * y),
            (&[x], []) => Some(x),
            _ => None,
        }
    }

    fn but_not(mut self, part1: &Self) -> Self {
        self.h_splits
            .retain(|h_split| !part1.h_splits.contains(h_split));
        self.v_splits
            .retain(|v_split| !part1.v_splits.contains(v_split));
        self
    }
}

fn v_split(data: &[impl Deref<Target = [u8]>]) -> Vec<usize> {
    let w = data[0].len();
    (1..w)
        .flat_map(|v_split| {
            for row in data {
                let len = std::cmp::min(v_split, w - v_split);
                if !eq_rev(
                    row[v_split - len..v_split].iter(),
                    row[v_split..v_split + len].iter(),
                ) {
                    return None;
                }
            }
            Some(v_split)
        })
        .collect_vec()
}

fn h_split(data: &[impl Deref<Target = [u8]>]) -> Vec<usize> {
    let h = data.len();
    let w = data[0].len();
    (1..h)
        .flat_map(|h_split| {
            for x in 0..w {
                let len = std::cmp::min(h_split, h - h_split);
                if !eq_rev(
                    (h_split - len..h_split).map(|y| data[y][x]),
                    (h_split..h_split + len).map(|y| data[y][x]),
                ) {
                    return None;
                }
            }
            Some(h_split)
        })
        .collect_vec()
}

fn solve(data: &[impl Deref<Target = [u8]>]) -> SplitLines {
    SplitLines {
        v_splits: v_split(data),
        h_splits: h_split(data),
    }
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let data = data.split(|line| line.is_empty()).collect_vec();
    data.iter()
        .map(|pattern| solve(pattern).summarize().unwrap())
        .sum()
}

fn part_2(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let data = data.split(|line| line.is_empty()).collect_vec();
    data.iter()
        .map(|pattern| {
            let w = pattern[0].len();
            let h = pattern.len();
            let part1 = solve(pattern);

            let mut pattern: Vec<Vec<u8>> = pattern.iter().copied().map(Vec::from).collect_vec();
            let invert = |n: u8| -> u8 {
                match n {
                    b'#' => b'.',
                    _ => b'#',
                }
            };

            for y in 0..h {
                for x in 0..w {
                    pattern[y][x] = invert(pattern[y][x]);
                    let splits = solve(&pattern[..]);
                    if splits != part1 {
                        if let Some(val) = splits.but_not(&part1).summarize() {
                            return val;
                        }
                    }
                    pattern[y][x] = invert(pattern[y][x]);
                }
            }
            unreachable!()
        })
        .sum()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:13:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:13:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(part_1(input), 405);
    dbg!();
    assert_eq!(part_2(input), 400);
}
