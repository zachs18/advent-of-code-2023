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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Pipe {
    Source,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Pipe {
    fn from_u8(b: u8) -> Option<Self> {
        match b {
            b'S' => Some(Self::Source),
            b'|' => Some(Self::Vertical),
            b'-' => Some(Self::Horizontal),
            b'7' => Some(Self::SouthWest),
            b'F' => Some(Self::SouthEast),
            b'L' => Some(Self::NorthEast),
            b'J' => Some(Self::NorthWest),
            _ => None,
        }
    }

    fn go_through(&self, incoming: Direction) -> Option<Direction> {
        use Direction::*;
        match (self, incoming) {
            (Pipe::Source, _) => unreachable!("should not call this on Source"),
            (Pipe::Vertical, src @ (North | South)) => Some(src),
            (Pipe::Horizontal, src @ (East | West)) => Some(src),
            (Pipe::NorthEast, South) => Some(East),
            (Pipe::NorthEast, West) => Some(North),
            (Pipe::NorthWest, South) => Some(West),
            (Pipe::NorthWest, East) => Some(North),
            (Pipe::SouthEast, North) => Some(East),
            (Pipe::SouthEast, West) => Some(South),
            (Pipe::SouthWest, North) => Some(West),
            (Pipe::SouthWest, East) => Some(South),

            (Pipe::NorthEast, North | East) => None,
            (Pipe::Vertical, East | West) => None,
            (Pipe::Horizontal, North | South) => None,
            (Pipe::NorthWest, North | West) => None,
            (Pipe::SouthEast, South | East) => None,
            (Pipe::SouthWest, South | West) => None,
        }
    }
}

/// Parses the map, calculates the maximum distance along the loop,
/// and clears the non-loop elements from the map.
fn parse_etc(input: &str) -> (Vec<Box<[Option<Pipe>]>>, usize) {
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

    let mut data: Vec<Box<[Option<Pipe>]>> = data
        .into_iter()
        .map(|row| row.iter().map(|&b| Pipe::from_u8(b)).collect())
        .collect_vec();

    let mut max_distance = 0;

    for row in 0..h {
        for col in 0..w {
            if distances[row][col] == usize::MAX {
                data[row][col] = None;
            } else {
                max_distance = Ord::max(distances[row][col], max_distance);
            }
        }
    }

    // replace S by actual pipe
    let north = (s_row > 0)
        .then(|| data[s_row - 1][s_col])
        .flatten()
        .filter(|p| p.go_through(Direction::North).is_some());
    let south = (s_row < h - 1)
        .then(|| data[s_row + 1][s_col])
        .flatten()
        .filter(|p| p.go_through(Direction::South).is_some());
    let west = (s_col > 0)
        .then(|| data[s_row][s_col - 1])
        .flatten()
        .filter(|p| p.go_through(Direction::West).is_some());
    let east = (s_col < w - 1)
        .then(|| data[s_row][s_col + 1])
        .flatten()
        .filter(|p| p.go_through(Direction::East).is_some());

    let s_pipe = match (
        north.is_some(),
        south.is_some(),
        west.is_some(),
        east.is_some(),
    ) {
        (true, true, false, false) => Pipe::Vertical,
        (true, false, true, false) => Pipe::NorthWest,
        (true, false, false, true) => Pipe::NorthEast,
        (false, true, true, false) => Pipe::SouthWest,
        (false, true, false, true) => Pipe::SouthEast,
        (false, false, true, true) => Pipe::Horizontal,
        adj => unreachable!("invalid S adjacency: {adj:?}"),
    };
    data[s_row][s_col] = Some(s_pipe);

    (data, max_distance)
}

fn part_1(&(_, max_distance): &(Vec<Box<[Option<Pipe>]>>, usize)) -> usize {
    max_distance
}

fn part_2((map, _): &(Vec<Box<[Option<Pipe>]>>, usize)) -> usize {
    let mut inner_cells = 0;

    for row in map {
        let mut crosses = 0_usize;
        let mut partial_cross_src_was_north = false;
        for cell in row.iter().copied() {
            match cell {
                None => {
                    if crosses % 2 != 0 {
                        inner_cells += 1;
                    }
                }
                Some(Pipe::Vertical) => crosses += 1,
                Some(Pipe::Horizontal) => {}
                Some(Pipe::NorthEast) => partial_cross_src_was_north = true,
                Some(Pipe::SouthEast) => partial_cross_src_was_north = false,
                Some(Pipe::NorthWest) => {
                    if !partial_cross_src_was_north {
                        crosses += 1;
                    }
                }
                Some(Pipe::SouthWest) => {
                    if partial_cross_src_was_north {
                        crosses += 1;
                    }
                }
                Some(p) => unreachable!("{p:?} should not be in the map at this point"),
            }
        }
    }

    inner_cells
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let session = session.trim();
    let mut both = PreParsed::new(parse_etc, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:10:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
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
    assert_eq!(part_1(&parse_etc(input)), 4);

    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(part_1(&parse_etc(input)), 8);

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
    assert_eq!(part_2(&parse_etc(input)), 8);
}
