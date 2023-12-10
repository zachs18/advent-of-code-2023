#![allow(unused_imports)]
use std::{collections::VecDeque, ops::ControlFlow};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let s_loc @ (s_row, s_col) = data
        .iter()
        .enumerate()
        .find_map(|(rowidx, row)| {
            let colidx = row.iter().position(|&b| b == b'S')?;
            Some((rowidx, colidx))
        })
        .unwrap();
    let h = data.len();
    let w = data[0].len();
    let mut distances = vec![vec![usize::MAX; w]; h];
    distances[s_row][s_col] = 0;
    let mut frontier = VecDeque::from([
        (s_loc, Direction::South, 0_usize),
        (s_loc, Direction::North, 0),
        (s_loc, Direction::West, 0),
        (s_loc, Direction::East, 0),
    ]);
    while let Some((loc, dir, dist)) = frontier.pop_front() {
        let (y, x) = loc;
        let (y2, x2) = match dir {
            Direction::North => (y.wrapping_sub(1), x),
            Direction::South => (y + 1, x),
            Direction::West => (y, x.wrapping_sub(1)),
            Direction::East => (y, x + 1),
        };
        if y2 >= h || x2 >= w {
            continue;
        }
        let newdist = dist + 1;
        if newdist < distances[y2][x2] {
            let cell = data[y2][x2];
            let newdir = match (cell, dir) {
                (b'-', Direction::West) => Direction::West,
                (b'-', Direction::East) => Direction::East,
                (b'|', Direction::North) => Direction::North,
                (b'|', Direction::South) => Direction::South,
                (b'7', Direction::North) => Direction::West,
                (b'7', Direction::East) => Direction::South,
                (b'L', Direction::South) => Direction::East,
                (b'L', Direction::West) => Direction::North,
                (b'J', Direction::South) => Direction::West,
                (b'J', Direction::East) => Direction::North,
                (b'F', Direction::North) => Direction::East,
                (b'F', Direction::West) => Direction::South,
                _ => continue,
            };

            distances[y2][x2] = newdist;
            frontier.push_back(((y2, x2), newdir, newdist));
        }
    }
    distances
        .into_iter()
        .flatten()
        .filter(|&dist| dist < usize::MAX)
        .max()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();
    let s_loc @ (s_row, s_col) = data
        .iter()
        .enumerate()
        .find_map(|(rowidx, row)| {
            let colidx = row.iter().position(|&b| b == b'S')?;
            Some((rowidx, colidx))
        })
        .unwrap();
    let h = data.len();
    let w = data[0].len();
    let mut distances = vec![vec![usize::MAX; w]; h];
    distances[s_row][s_col] = 0;
    let mut frontier = VecDeque::from([
        (s_loc, Direction::South, 0_usize),
        (s_loc, Direction::North, 0),
        (s_loc, Direction::West, 0),
        (s_loc, Direction::East, 0),
    ]);
    while let Some((loc, dir, dist)) = frontier.pop_front() {
        let (y, x) = loc;
        let (y2, x2) = match dir {
            Direction::North => (y.wrapping_sub(1), x),
            Direction::South => (y + 1, x),
            Direction::West => (y, x.wrapping_sub(1)),
            Direction::East => (y, x + 1),
        };
        if y2 >= h || x2 >= w {
            continue;
        }
        let newdist = dist + 1;
        if newdist < distances[y2][x2] {
            let cell = data[y2][x2];
            let newdir = match (cell, dir) {
                (b'-', Direction::West) => Direction::West,
                (b'-', Direction::East) => Direction::East,
                (b'|', Direction::North) => Direction::North,
                (b'|', Direction::South) => Direction::South,
                (b'7', Direction::North) => Direction::West,
                (b'7', Direction::East) => Direction::South,
                (b'L', Direction::South) => Direction::East,
                (b'L', Direction::West) => Direction::North,
                (b'J', Direction::South) => Direction::West,
                (b'J', Direction::East) => Direction::North,
                (b'F', Direction::North) => Direction::East,
                (b'F', Direction::West) => Direction::South,
                _ => continue,
            };

            distances[y2][x2] = newdist;
            frontier.push_back(((y2, x2), newdir, newdist));
        }
    }

    let mut data = data.into_iter().map(<Box<[u8]>>::from).collect_vec();

    for row in 0..h {
        for col in 0..w {
            if distances[row][col] == usize::MAX {
                data[row][col] = b'.';
            }
        }
    }

    // replace S by actual pipe
    let north = (s_row > 0)
        .then(|| data[s_row - 1][s_col])
        .filter(|b| matches!(b, b'|' | b'F' | b'7'));
    let south = (s_row < h - 1)
        .then(|| data[s_row + 1][s_col])
        .filter(|b| matches!(b, b'|' | b'L' | b'J'));
    let west = (s_col > 0)
        .then(|| data[s_row][s_col - 1])
        .filter(|b| matches!(b, b'-' | b'F' | b'L'));
    let east = (s_col < w - 1)
        .then(|| data[s_row][s_col + 1])
        .filter(|b| matches!(b, b'-' | b'7' | b'J'));

    let s_pipe = match (
        north.is_some(),
        south.is_some(),
        west.is_some(),
        east.is_some(),
    ) {
        (true, true, true, true) => unreachable!(),
        (true, true, true, false) => unreachable!(),
        (true, true, false, true) => unreachable!(),
        (true, true, false, false) => b'|',
        (true, false, true, true) => unreachable!(),
        (true, false, true, false) => b'J',
        (true, false, false, true) => b'L',
        (true, false, false, false) => unreachable!(),
        (false, true, true, true) => unreachable!(),
        (false, true, true, false) => b'7',
        (false, true, false, true) => b'F',
        (false, true, false, false) => unreachable!(),
        (false, false, true, true) => b'-',
        (false, false, true, false) => unreachable!(),
        (false, false, false, true) => unreachable!(),
        (false, false, false, false) => unreachable!(),
    };
    data[s_row][s_col] = s_pipe;

    let mut inner_cells = 0;

    for row in 0..h {
        let mut crosses = 0_usize;
        let mut partial_cross_src_was_up = false;
        for col in 0..w {
            match data[row][col] {
                b'.' => {
                    if crosses % 2 != 0 {
                        inner_cells += 1;
                        data[row][col] = b'X';
                    }
                }
                b'|' => crosses += 1,
                b'-' => {}
                b'L' => partial_cross_src_was_up = true,
                b'F' => partial_cross_src_was_up = false,
                b'J' => {
                    if !partial_cross_src_was_up {
                        crosses += 1;
                    }
                }
                b'7' => {
                    if partial_cross_src_was_up {
                        crosses += 1;
                    }
                }
                b => unreachable!("{} should not be in the map at this point", b as char),
            }
        }
    }

    for row in &data {
        for &cell in row.iter() {
            eprint!("{}", cell as char);
        }
        eprintln!();
    }

    inner_cells
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:10:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:10:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
    assert_eq!(part_1(input), 4);

    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(part_1(input), 8);

    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    assert_eq!(part_2(input), 8);
}
