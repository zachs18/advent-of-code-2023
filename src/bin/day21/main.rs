#![allow(unused_imports)]
use std::collections::{HashMap, HashSet};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let (y, x) = data
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == b'S').map(|x| (y, x)))
        .unwrap();
    let h = data.len();
    let w = data[0].len();
    let mut possible: HashSet<(usize, usize)> = HashSet::from([(y, x)]);
    const COUNT: usize = {
        #[cfg(test)]
        {
            6
        }
        #[cfg(not(test))]
        {
            64
        }
    };
    for _ in 0..COUNT {
        let mut new_possible = HashSet::new();
        for (y, x) in possible {
            if y.wrapping_sub(1) < h && data[y - 1][x] != b'#' {
                new_possible.insert((y - 1, x));
            }
            if x.wrapping_sub(1) < w && data[y][x - 1] != b'#' {
                new_possible.insert((y, x - 1));
            }
            if y + 1 < h && data[y + 1][x] != b'#' {
                new_possible.insert((y + 1, x));
            }
            if x + 1 < w && data[y][x + 1] != b'#' {
                new_possible.insert((y, x + 1));
            }
        }
        possible = new_possible;
    }
    possible.len()
}

fn part_2(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let (y, x) = data
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == b'S').map(|x| (y, x)))
        .unwrap();
    let h = data.len();
    let w = data[0].len();
    let mut possible: HashSet<(isize, isize)> = HashSet::from([((y as isize, x as isize))]);
    const COUNT: usize = {
        #[cfg(test)]
        {
            6
        }
        #[cfg(not(test))]
        {
            26501365
        }
    };
    for i in 0..COUNT {
        dbg!(i, possible.len());
        let mut new_possible = HashSet::new();
        for (y, x) in possible {
            let cy = y.rem_euclid(h as isize) as usize;
            let cx = x.rem_euclid(w as isize) as usize;
            {
                let y = y - 1;
                let cy = cy.checked_sub(1).unwrap_or(h - 1);
                if data[cy][cx] != b'#' {
                    new_possible.insert((y, x));
                }
            }
            {
                let y = y + 1;
                let cy = (cy + 1) % h;
                if data[cy][cx] != b'#' {
                    new_possible.insert((y, x));
                }
            }
            {
                let x = x - 1;
                let cx = cx.checked_sub(1).unwrap_or(w - 1);
                if data[cy][cx] != b'#' {
                    new_possible.insert((y, x));
                }
            }
            {
                let x = x + 1;
                let cx = (cx + 1) % w;
                if data[cy][cx] != b'#' {
                    new_possible.insert((y, x));
                }
            }
        }
        possible = new_possible;
    }
    possible.len()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:21:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:21:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    assert_eq!(part_1(input), 42);
    assert_eq!(part_2(input), 42);
}
