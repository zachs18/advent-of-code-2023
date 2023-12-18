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

enum Trench {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    East,
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
    (a + b + c)
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
    /// number of 90 degree clockwise rotations
    fn rotation_from(self, from: Self) -> isize {
        use Direction::*;
        match (self, from) {
            (North, North) | (East, East) | (South, South) | (West, West) => 0,
            (North, East) | (East, South) | (South, West) | (West, North) => 1,
            (East, North) | (South, East) | (West, South) | (North, West) => -1,
            _ => unreachable!("{self:?} {from:?}"),
        }
    }

    fn trench(self) -> Trench {
        match self {
            Direction::North => Trench::North,
            Direction::East => Trench::East,
            Direction::South => Trench::South,
            Direction::West => Trench::West,
        }
    }

    /// (turn, straight)
    /// turn should overwrite the current location
    fn trench_from(self, prev: Self) -> (Trench, Trench) {
        use Direction::*;
        let straight = self.trench();
        let turn = match (self, prev) {
            (North | South, North | South) | (East | West, East | West) => {
                unreachable!("not a turn")
            }
            (North, East) | (East, North) => Trench::NorthEast,
            (South, East) | (East, South) => Trench::SouthEast,
            (North, West) | (West, North) => Trench::NorthWest,
            (South, West) | (West, South) => Trench::SouthWest,
        };
        (turn, straight)
    }
}

fn calculate_area(path: &[(Direction, usize)]) -> usize {
    let mut total_side_length: usize = 0;
    let mut current_signed_area_in_halves: isize = 0;
    let mut prev_point: Point = (0, 0).into();
    for &(direction, dist) in path {
        total_side_length += dist;
        let current_point = prev_point + direction.offset() * dist as isize;
        let a = signed_area_clockwise_from_origin_in_halves(prev_point, current_point);
        current_signed_area_in_halves += a;
        prev_point = current_point;
    }

    current_signed_area_in_halves.abs_diff(0) / 2 + total_side_length / 2 + 1
}

fn part_1(input: &str) -> usize {
    let path = input
        .lines()
        .map(str::trim)
        .map(|line| {
            let mut fields = line.split_whitespace();
            let direction = match fields.next().unwrap() {
                "R" => Direction::East,
                "U" => Direction::North,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => unreachable!(),
            };
            let count = fields.next().unwrap().parse::<usize>().unwrap();
            (direction, count)
        })
        .collect_vec();
    calculate_area(&path)
}

#[cfg(any())]
fn part_1_aaa(input: &str) -> usize {
    let path = input
        .lines()
        .map(str::trim)
        .map(|line| {
            let mut fields = line.split_whitespace();
            let direction = match fields.next().unwrap() {
                "R" => Direction::East,
                "U" => Direction::North,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => unreachable!(),
            };
            let count = fields.next().unwrap().parse::<isize>().unwrap();
            (direction, count)
        })
        .collect_vec();
    let mut map: BTreeMap<isize, BTreeMap<isize, Trench>> =
        BTreeMap::from([(0, BTreeSet::from([0]))]);
    let mut y = 0;
    let mut x = 0;
    let mut prev_dir = path.last().unwrap().0;

    for (direction, length) in path {
        let (dy, dx) = direction.offset();
        prev_dir = direction;
        for _ in 0..length {
            y += dy;
            x += dx;
            map.entry(y).or_default().insert(x);
        }
    }

    let &miny = map.first_key_value().unwrap().0;
    let &minx = map.values().map(|row| row.first().unwrap()).min().unwrap();
    let &maxy = map.last_key_value().unwrap().0;
    let &maxx = map.values().map(|row| row.last().unwrap()).max().unwrap();

    let w = (maxx - minx + 1) as usize;
    let h = (maxy - miny + 1) as usize;

    let mut maap = vec![vec![None::<Trench>; w]; h];

    for (y, row) in map {
        for x in row {
            maap[(y - miny) as usize][(x - minx) as usize] = Some(Cell::Trench);
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
    let (path1, path2): (Vec<(Direction, usize)>, Vec<(Direction, usize)>) = input
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
    calculate_area(&path2)
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
    assert_eq!(part_2(input), 952408144115);
}
