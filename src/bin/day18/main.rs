#![allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn offset(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
    /// number of 90 degree clockwise rotations
    fn rotation_from(self, from: Self) -> isize {
        use Direction::*;
        match (self, from) {
            (Up, Up) | (Right, Right) | (Down, Down) | (Left, Left) => 0,
            (Up, Right) | (Right, Down) | (Down, Left) | (Left, Up) => 1,
            (Right, Up) | (Down, Right) | (Left, Down) | (Up, Left) => -1,
            _ => unreachable!("{self:?} {from:?}"),
        }
    }
}

fn part_1(input: &str) -> usize {
    let path = input
        .lines()
        .map(str::trim)
        .map(|line| {
            let mut fields = line.split_whitespace();
            let direction = match fields.next().unwrap() {
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => unreachable!(),
            };
            let count = fields.next().unwrap().parse::<isize>().unwrap();
            (direction, count)
        })
        .collect_vec();
    let mut map: BTreeMap<isize, BTreeSet<isize>> = BTreeMap::from([(0, BTreeSet::from([0]))]);
    let mut y = 0;
    let mut x = 0;
    let mut prev_dir = path[0].0;

    let mut total_rotations = 0;

    for (direction, length) in path {
        let (dy, dx) = direction.offset();
        total_rotations += dbg!(direction.rotation_from(prev_dir));
        prev_dir = direction;
        for _ in 0..length {
            y += dy;
            x += dx;
            map.entry(y).or_default().insert(x);
        }
    }
    // panic!("{total_rotations}");

    let &miny = map.first_key_value().unwrap().0;
    let &minx = map.values().map(|row| row.first().unwrap()).min().unwrap();
    let &maxy = map.last_key_value().unwrap().0;
    let &maxx = map.values().map(|row| row.last().unwrap()).max().unwrap();

    let w = (maxx - minx + 1) as usize;
    let h = (maxy - miny + 1) as usize;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Cell {
        Trench,
        Outside,
        Unknown,
    }

    let mut maap = vec![vec![Cell::Unknown; w]; h];

    for (y, row) in map {
        for x in row {
            maap[(y - miny) as usize][(x - minx) as usize] = Cell::Trench;
        }
    }

    // flood fill Outside from the edges
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut add_if_not_trench = |y: usize, x: usize| {
        if maap[y][x] != Cell::Trench {
            maap[y][x] = Cell::Outside;
            queue.push_back((y, x));
        }
    };
    for y in 0..h {
        add_if_not_trench(y, 0);
        add_if_not_trench(y, w - 1);
    }
    for x in 0..w {
        add_if_not_trench(0, x);
        add_if_not_trench(h - 1, x);
    }

    while let Some((y, x)) = queue.pop_front() {
        let mut add_if_not_trench = |y: usize, x: usize| {
            if maap[y][x] == Cell::Unknown {
                maap[y][x] = Cell::Outside;
                queue.push_back((y, x));
            }
        };
        if y > 0 {
            add_if_not_trench(y - 1, x);
        }
        if y + 1 < h {
            add_if_not_trench(y + 1, x);
        }
        if x > 0 {
            add_if_not_trench(y, x - 1);
        }
        if x + 1 < w {
            add_if_not_trench(y, x + 1);
        }
    }

    maap.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| (cell != Cell::Outside) as usize)
                .sum::<usize>()
        })
        .sum::<usize>()

    // find a cell inside
    // for (y, row) in maap.iter().enumerate() {
    //     let mut inside
    // }

    // assert!((y, x) == (0, 0));
    // let mut acc = 0;
    // for (y, row) in map {
    //     let mut is_inside = false;
    //     let mut prev_x = isize::MIN;
    //     print!("{}: ", row.first().unwrap());
    //     for x in row {
    //         print!("#");
    //         if x != prev_x + 1 {
    //             if is_inside {
    //                 acc += (x - prev_x - 1) as usize;
    //                 print!("{:w$}", "", w = (x - prev_x - 1) as usize);
    //             }
    //             is_inside = !is_inside;
    //         }
    //         acc += 1;
    //         prev_x = x;
    //     }
    //     println!();
    // }
    // acc
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:18:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
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
    assert_eq!(part_1(input), 62);
    assert_eq!(part_2(input), 42);
}
