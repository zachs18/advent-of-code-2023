#![allow(unused_imports)]
use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

fn eq_rev<T: Eq>(s1: impl Iterator<Item = T>, s2: impl DoubleEndedIterator<Item = T>) -> bool {
    s1.eq(s2.rev())
}

fn solve(data: &[&[u8]]) -> usize {
    let h = data.len();
    let w = data[0].len();
    'v_split: for v_split in 1..w {
        for &row in data {
            let len = std::cmp::min(v_split, w - v_split);
            if !eq_rev(
                row[v_split - len..v_split].iter(),
                row[v_split..v_split + len].iter(),
            ) {
                continue 'v_split;
            }
        }
        return v_split;
    }
    'h_split: for h_split in 1..h {
        for x in 0..w {
            let len = std::cmp::min(h_split, h - h_split);
            if !eq_rev(
                (h_split - len..h_split).map(|y| data[y][x]),
                (h_split..h_split + len).map(|y| data[y][x]),
            ) {
                continue 'h_split;
            }
        }
        return 100 * h_split;
    }
    unreachable!()
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let data = data.split(|line| line.is_empty()).collect_vec();
    data.iter().map(|pattern| solve(&pattern)).sum()
}

fn part_2(input: &str) -> u64 {
    todo!()
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
    assert_eq!(part_2(input), 400);
}
