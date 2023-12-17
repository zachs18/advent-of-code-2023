#![allow(unused_imports)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    heat_loss: usize,
    position: (u8, u8),
    last_move_direction: Direction,
    last_move_count: u8,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss
            .cmp(&other.heat_loss)
            .reverse()
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.last_move_direction.cmp(&other.last_move_direction))
            .then_with(|| self.last_move_count.cmp(&other.last_move_count))
    }
}

impl State {
    fn and_move(mut self, map: &[Vec<u8>], direction: Direction) -> Option<Self> {
        if self.last_move_direction == direction.opposite() {
            return None;
        } else if self.last_move_direction == direction && self.last_move_count >= 3 {
            return None;
        }
        let h = map.len() as u8;
        let w = map[0].len() as u8;
        let (y, x) = self.position;
        let (y, x) = match direction {
            Direction::North => (y.checked_sub(1)?, x),
            Direction::East => (y, x + 1),
            Direction::South => (y + 1, x),
            Direction::West => (y, x.checked_sub(1)?),
        };
        if y >= h || x >= w {
            return None;
        }
        if self.last_move_direction == direction {
            self.last_move_count += 1;
        } else {
            self.last_move_direction = direction;
            self.last_move_count = 1;
        }
        self.heat_loss += map[y as usize][x as usize] as usize;
        self.position = (y, x);
        Some(self)
    }
}

fn part_1(input: &str) -> usize {
    let map = input
        .lines()
        .map(str::trim)
        .map(|line| line.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec();
    let h = map.len();
    let w = map[0].len();

    let state = State {
        heat_loss: 0,
        position: (0, 0),
        last_move_direction: Direction::East,
        last_move_count: 0,
    };

    let mut queue = BinaryHeap::from([state]);
    // (y, x, dir, dircount)
    let mut best_heat_loss: HashMap<((u8, u8), Direction, u8), usize> = HashMap::new();
    while let Some(state) = queue.pop() {
        if state.position == ((h - 1) as u8, (w - 1) as u8) {
            return state.heat_loss;
        } else if let Some(best_heat_loss) = best_heat_loss.get_mut(&(
            state.position,
            state.last_move_direction,
            state.last_move_count,
        )) {
            if state.heat_loss < *best_heat_loss {
                *best_heat_loss = state.heat_loss;
            } else {
                continue;
            }
        } else {
            best_heat_loss.insert(
                (
                    state.position,
                    state.last_move_direction,
                    state.last_move_count,
                ),
                state.heat_loss,
            );
        }
        static mut COUNTER: u32 = 0;
        unsafe {
            if COUNTER % 0x100000 == 0 {
                dbg!(state);
            }
            COUNTER = COUNTER.wrapping_add(1);
        }
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if let Some(state) = state.and_move(&map, direction) {
                queue.push(state);
            }
        }
    }

    todo!()
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
