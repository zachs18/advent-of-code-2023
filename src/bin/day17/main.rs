#![allow(unused_imports)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    f32::consts::E,
    rc::Rc,
};

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

impl Direction {
    const fn opposite(self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    path: Vec<(u8, u8)>,
    heat_loss: usize,
    last_direction: Direction,
    consecutive_moves: usize,
}

impl Path {
    fn and_move(&self, map: &[Vec<u8>], direction: Direction) -> Option<Self> {
        let h = map.len() as u8;
        let w = map[0].len() as u8;
        let (y, x) = self.path.last().copied().unwrap();
        let (y, x) = match direction {
            Direction::North => (y.checked_sub(1)?, x),
            Direction::East => (y, x + 1),
            Direction::South => (y + 1, x),
            Direction::West => (y, x.checked_sub(1)?),
        };
        if y >= h || x >= w {
            return None;
        }
        //  else if self.path.contains(&(y, x)) {
        //     // note: this implements "cannot reverse" logic
        //     return None;
        // }

        ((self.last_direction != direction || self.consecutive_moves < 3)
            && self.last_direction != direction.opposite())
        .then(|| {
            let mut path = self.clone();
            if self.last_direction != direction {
                path.last_direction = direction;
                path.consecutive_moves = 1
            } else {
                path.consecutive_moves += 1;
            }
            path.path.push((y, x));
            path.heat_loss += map[y as usize][x as usize] as usize;
            path
        })
    }

    fn print(&self, map: &[Vec<u8>]) {
        let h = map.len();
        let w = map[0].len();
        for y in 0..h {
            for x in 0..w {
                let path_idx = self.path.iter().position(|&pos| pos == (y as u8, x as u8));
                match path_idx {
                    Some(path_idx) => match self.path.get(path_idx.saturating_sub(1)..path_idx + 1)
                    {
                        Some(&[(y1, x1), (y2, x2)]) => {
                            match (y2 as i32 - y1 as i32, x2 as i32 - x1 as i32) {
                                (-1, 0) => print!("^"),
                                (1, 0) => print!("v"),
                                (0, -1) => print!("<"),
                                (0, 1) => print!(">"),
                                _ => unreachable!(),
                            }
                        }
                        _ => print!("X"),
                    },
                    None => print!("{}", map[y][x]),
                }
            }
            println!();
        }
    }
}

const KEEP_PATHS: usize = 12;

fn part_1(input: &str) -> usize {
    let map = input
        .lines()
        .map(str::trim)
        .map(|line| line.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let h = map.len();
    let w = map[0].len();

    let path = Path {
        path: vec![(0, 0)],
        heat_loss: 0,
        last_direction: Direction::East,
        consecutive_moves: 0,
    };

    // keep track of the two best ways to get to any cell.
    // this will always have some way to get to adjacent cells that won't cause three
    // consecutive straight line movements.
    let mut bests: Vec<Vec<[Option<Path>; KEEP_PATHS]>> = vec![
        vec![
            {
                const NONE: Option<Path> = None;
                [NONE; KEEP_PATHS]
            };
            w
        ];
        h
    ];
    bests[0][0] = vec![Some(path.clone()); KEEP_PATHS].try_into().unwrap();

    let insert_path = |bests: &mut [Option<Path>; KEEP_PATHS], path: Path| -> bool {
        for idx in 0..KEEP_PATHS {
            match &bests[idx] {
                Some(best) => {
                    if path.heat_loss < best.heat_loss {
                        bests[idx..].rotate_right(1);
                        bests[idx] = Some(path);
                        return true;
                    } else if path == *best {
                        return false;
                    }
                }
                None => {
                    bests[idx] = Some(path);
                    return true;
                }
            }
        }
        false
    };

    let mut re_check_adjacent_to: Vec<(usize, usize)> = vec![(0, 0)];
    while !re_check_adjacent_to.is_empty() {
        let mut new_re_check_adjacent_to = vec![];
        for (y, x) in re_check_adjacent_to {
            if y > 0 {
                // check going north
                for path in bests[y][x]
                    .iter()
                    .flat_map(|path| path.as_ref()?.and_move(&map, Direction::North))
                    .collect_vec()
                {
                    if insert_path(&mut bests[y - 1][x], path) {
                        new_re_check_adjacent_to.push((y - 1, x));
                    }
                }
            }
            if y + 1 < h {
                // check going south
                for path in bests[y][x]
                    .iter()
                    .flat_map(|path| path.as_ref()?.and_move(&map, Direction::South))
                    .collect_vec()
                {
                    if insert_path(&mut bests[y + 1][x], path) {
                        new_re_check_adjacent_to.push((y + 1, x));
                    }
                }
            }
            if x > 0 {
                // check going west
                for path in bests[y][x]
                    .iter()
                    .flat_map(|path| path.as_ref()?.and_move(&map, Direction::West))
                    .collect_vec()
                {
                    if insert_path(&mut bests[y][x - 1], path) {
                        new_re_check_adjacent_to.push((y, x - 1));
                    }
                }
            }
            if x + 1 < w {
                // check going south
                for path in bests[y][x]
                    .iter()
                    .flat_map(|path| path.as_ref()?.and_move(&map, Direction::East))
                    .collect_vec()
                {
                    if insert_path(&mut bests[y][x + 1], path) {
                        new_re_check_adjacent_to.push((y, x + 1));
                    }
                }
            }
        }
        re_check_adjacent_to = new_re_check_adjacent_to;
    }

    let best_path = bests[h - 1][w - 1][0].as_ref().unwrap();
    best_path.print(&map);

    bests[h - 1][w - 1][0].as_ref().unwrap().heat_loss
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:17:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:17:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    assert_eq!(part_1(input), 102);
    // assert_eq!(part_2(input), 42);
}
