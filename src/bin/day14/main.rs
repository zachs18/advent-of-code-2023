#![allow(unused_imports)]
use std::{cell::Cell, collections::HashMap};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Space {
    RoundRock,
    CubeRock,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Platform {
    cells: Vec<Vec<Space>>,
}

impl Platform {
    fn tilt_north(&mut self) {
        for x in 0..self.cells[0].len() {
            let mut column = self.cells.iter_mut().map(|row| &mut row[x]).collect_vec();
            tilt(&mut column);
        }
    }
    fn tilt_south(&mut self) {
        for x in 0..self.cells[0].len() {
            let mut column = self
                .cells
                .iter_mut()
                .rev()
                .map(|row| &mut row[x])
                .collect_vec();
            tilt(&mut column);
        }
    }
    fn tilt_east(&mut self) {
        for row in &mut self.cells {
            let mut row = row.iter_mut().rev().collect_vec();
            tilt(&mut row);
        }
    }
    fn tilt_west(&mut self) {
        for row in &mut self.cells {
            let mut row = row.iter_mut().collect_vec();
            tilt(&mut row);
        }
    }

    fn calculate_load(&self) -> usize {
        let h = self.cells.len();
        let w = self.cells[0].len();
        (0..w)
            .map(|x| {
                (0..h)
                    .map(|y| {
                        if self.cells[y][x] == Space::RoundRock {
                            h - y
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn tilt(column: &mut [&mut Space]) {
    let mut read_head = 0;
    let mut write_head = 0;
    while read_head < column.len() {
        match column[read_head] {
            Space::RoundRock => {
                if read_head != write_head {
                    *column[write_head] = Space::RoundRock;
                    *column[read_head] = Space::Empty;
                }
                read_head += 1;
                write_head += 1;
            }
            Space::Empty => {
                read_head += 1;
            }
            Space::CubeRock => {
                read_head += 1;
                write_head = read_head;
            }
        }
    }
}

fn tilt_column_north(column: &mut [u8]) {
    let mut read_head = 0;
    let mut write_head = 0;
    while read_head < column.len() {
        match column[read_head] {
            b'O' => {
                if read_head != write_head {
                    column[write_head] = b'O';
                    column[read_head] = b'.';
                }
                read_head += 1;
                write_head += 1;
            }
            b'.' => {
                read_head += 1;
            }
            b'#' => {
                read_head += 1;
                write_head = read_head;
            }
            _ => unreachable!(),
        }
    }
}

fn calculate_column_load(column: &[u8]) -> usize {
    let n = column.len();
    column
        .iter()
        .enumerate()
        .map(|(y, &rock)| if rock == b'O' { n - y } else { 0 })
        .sum()
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let h = data.len();
    let w = data[0].len();
    let mut columns = (0..w)
        .map(|x| (0..h).map(|y| data[y][x]).collect_vec())
        .collect_vec();
    for column in &mut columns {
        tilt_column_north(column);
    }
    columns
        .into_iter()
        .map(|column| calculate_column_load(&column))
        .sum()
}

fn part_2(input: &str) -> usize {
    let cells = input
        .lines()
        .map(|line| {
            line.trim()
                .bytes()
                .map(|b| match b {
                    b'O' => Space::RoundRock,
                    b'.' => Space::Empty,
                    b'#' => Space::CubeRock,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();
    let mut platform = Platform { cells };

    // Map from platform to cycle index.
    let mut cache = HashMap::new();
    const CYCLE_COUNT: usize = 1_000_000_000;
    // Find the first repeated platform such that it's cycle index
    //  minus the cycle index of the platform it repeats
    //  is a multiple of the required end cycle count minus the cycle index of the platform it repeats.
    // Finding such a platform and cycle index means that the final platform will be this platform,
    //  so use it to calculate the load.
    let mut cycle = 0_usize;
    cache.insert(platform.clone(), cycle);
    loop {
        cycle += 1;
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();
        match cache.entry(platform.clone()) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                let prev_cycle = *entry.get();
                if (CYCLE_COUNT - prev_cycle) % (cycle - prev_cycle) == 0 {
                    return platform.calculate_load();
                }
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(cycle);
            }
        }
    }
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:14:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:14:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part_1(input), 136);
    assert_eq!(part_2(input), 64);
}
