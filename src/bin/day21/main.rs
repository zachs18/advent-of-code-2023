#![allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::atomic::AtomicUsize,
};

use aoc_2023::*;
use aoc_driver::*;
use enum_map::EnumMap;
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

static COUNT: AtomicUsize = AtomicUsize::new(26501365);

/// Assuming no cells are unreachable
fn count_odd_and_even_squares(data: &[&[u8]]) -> (usize, usize) {
    let (sy, sx) = data
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == b'S').map(|x| (y, x)))
        .unwrap();
    let h = data.len();
    let w = data[0].len();
    let mut odd = 0;
    let mut even = 0;
    for y in 0..h {
        for x in 0..w {
            if data[y][x] != b'#' {
                let y_parity = (y & 1) == (sy & 1);
                let x_parity = (x & 1) == (sx & 1);
                let parity = y_parity ^ x_parity;
                if !parity {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
        }
    }
    (odd, even)
}

enum Universe<'data> {
    FullyExplored,
    NotFullyExplored(Box<UnexploredUniverse<'data>>),
}

struct UnexploredUniverse<'data> {
    data: &'data [&'data [u8]],
    seen: Vec<Vec<bool>>,
    frontier: HashSet<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, enum_map::Enum, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

impl<'data> Universe<'data> {
    fn new(data: &'data [&'data [u8]]) -> Self {
        Universe::NotFullyExplored(Box::new(UnexploredUniverse {
            data,
            seen: data.iter().map(|row| vec![false; row.len()]).collect_vec(),
            frontier: HashSet::new(),
        }))
    }

    fn is_fully_explored(&self) -> bool {
        matches!(self, Universe::FullyExplored)
    }

    /// Returns `((y, x), universe_direction)` for cells to be added to the frontier in different universes
    fn step(&mut self) -> EnumMap<Direction, Vec<(usize, usize)>> {
        let Universe::NotFullyExplored(universe) = self else {
            return Default::default();
        };
        let UnexploredUniverse {
            data,
            ref mut seen,
            ref mut frontier,
        } = **universe;
        assert!(
            !frontier.is_empty(),
            "should have been marked as fully explored"
        );
        let h = data.len();
        let w = data[0].len();
        let mut new_frontier: HashSet<(usize, usize)> = HashSet::with_capacity(frontier.len() * 2);
        let mut multiverse_frontier: EnumMap<Direction, Vec<(usize, usize)>> = Default::default();

        for (y, x) in std::mem::take(frontier) {
            seen[y][x] = true;
            {
                // North
                match y.checked_sub(1) {
                    Some(y) => {
                        if !seen[y][x] && data[y][x] != b'#' {
                            new_frontier.insert((y, x));
                        }
                    }
                    None => {
                        multiverse_frontier[Direction::North].push((h - 1, x));
                    }
                }
            }
            {
                // West
                match x.checked_sub(1) {
                    Some(x) => {
                        if !seen[y][x] && data[y][x] != b'#' {
                            new_frontier.insert((y, x));
                        }
                    }
                    None => {
                        multiverse_frontier[Direction::West].push((y, w - 1));
                    }
                }
            }
            {
                // South
                let y = y + 1;
                if y < h {
                    if !seen[y][x] && data[y][x] != b'#' {
                        new_frontier.insert((y, x));
                    }
                } else {
                    multiverse_frontier[Direction::South].push((0, x));
                }
            }
            {
                // East
                let x = x + 1;
                if x < w {
                    if !seen[y][x] && data[y][x] != b'#' {
                        new_frontier.insert((y, x));
                    }
                } else {
                    multiverse_frontier[Direction::East].push((y, 0));
                }
            }
        }
        *frontier = new_frontier;
        // dbg!(frontier.len());
        if frontier.is_empty() {
            *self = Universe::FullyExplored;
        }
        multiverse_frontier
    }

    fn append_frontier(&mut self, frontier: impl IntoIterator<Item = (usize, usize)>) {
        let Universe::NotFullyExplored(universe) = self else {
            return;
        };
        universe.frontier.extend(frontier);
    }
}

fn print_universe(data: &[&[u8]], uy: isize, ux: isize, universe: &Universe<'_>) {
    let h = data.len();
    let w = data[0].len();
    eprintln!("Universe {uy},{ux}:");
    match universe {
        Universe::FullyExplored => eprintln!("<fully explored>"),
        Universe::NotFullyExplored(universe) => {
            let ref seen = universe.seen[..];
            for y in 0..h {
                for x in 0..w {
                    match (data[y][x], seen[y][x]) {
                        (b'#', _) => eprint!("#"),
                        (b'S' | b'.', true) => eprint!("O"),
                        (b'S' | b'.', false) => eprint!("."),
                        _ => unreachable!(),
                    }
                }
                eprintln!();
            }
        }
    }
}

/// flip the results depending on the parity of the starting position.
fn count_reachable_odd_and_even(
    data: &[&[u8]],
    universes: &HashMap<(isize, isize), Universe<'_>>,
) -> (usize, usize) {
    // both the example input and my input have odd width and height, so I'm just hard-coding that.
    let h = data.len();
    let w = data[0].len();
    assert!(h % 2 == 1);
    assert!(w % 2 == 1);

    let (odd_parity_reachable_squares, even_parity_reachable_squares) =
        count_odd_and_even_squares(&data);
    let total_reachable_squares = odd_parity_reachable_squares + even_parity_reachable_squares;

    let mut odd = 0;
    let mut even = 0;

    for (&(uy, ux), universe) in universes {
        // print_universe(data, uy, ux, universe);

        let universe_parity = (uy & 1) != (ux & 1);
        let (oprs, eprs) = if let Universe::NotFullyExplored(universe) = universe {
            let mut oprs = 0;
            let mut eprs = 0;
            for y in 0..h {
                for x in 0..w {
                    let cell_parity = (x & 1) != (y & 1);
                    if universe.seen[y][x] {
                        if !cell_parity {
                            eprs += 1
                        } else {
                            oprs += 1;
                        }
                    }
                }
            }
            (oprs, eprs)
        } else {
            (odd_parity_reachable_squares, even_parity_reachable_squares)
        };
        if !universe_parity {
            odd += oprs;
            even += eprs;
        } else {
            even += oprs;
            odd += eprs;
        }
    }

    (odd, even)
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

    let mut universes: HashMap<(isize, isize), Universe<'_>> = HashMap::from([((0, 0), {
        let mut universe = Universe::new(&data);
        universe.append_frontier([(y, x)]);
        universe
    })]);

    let mut frontier: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);

    // ..= because the first step is the one that actually places the starting position.
    for i in 0..=COUNT.load(std::sync::atomic::Ordering::Relaxed) {
        // print_universe(&data, 0, 0, universes.get(&(0, 0)).unwrap());

        if i % 128 == 0 {
            eprintln!("TODO: exploit the fact the the real input has straight-line paths to the edges from the starting position, and that the edges are all clear");
            dbg!(i, frontier.len(), universes.len());
        }

        let mut new_frontier = HashSet::with_capacity(frontier.len());
        let mut to_add_multiverse_frontiers: HashMap<(isize, isize), Vec<(usize, usize)>> =
            HashMap::new();
        for (uy, ux) in frontier {
            let universe = universes.get_mut(&(uy, ux)).expect("universe exists");
            let multiverse_frontier = universe.step();
            if !universe.is_fully_explored() {
                new_frontier.insert((uy, ux));
            }
            for (dir, frontier) in multiverse_frontier {
                if !frontier.is_empty() {
                    let (dy, dx) = dir.as_offset();
                    let uy = uy + dy;
                    let ux = ux + dx;
                    match to_add_multiverse_frontiers.entry((uy, ux)) {
                        std::collections::hash_map::Entry::Occupied(entry) => {
                            entry.into_mut().extend(frontier);
                        }
                        std::collections::hash_map::Entry::Vacant(entry) => {
                            entry.insert(frontier);
                        }
                    }
                }
            }
        }

        for ((uy, ux), frontier) in to_add_multiverse_frontiers {
            if !frontier.is_empty() {
                let adjacent_universe = universes
                    .entry((uy, ux))
                    .or_insert_with(|| Universe::new(&data));
                adjacent_universe.append_frontier(frontier);
                if !adjacent_universe.is_fully_explored() {
                    new_frontier.insert((uy, ux));
                }
            }
        }

        frontier = new_frontier;
    }
    // print_universe(&data, 0, 0, universes.get(&(0, 0)).unwrap());
    dbg!(count_reachable_odd_and_even(&data, &universes)).1
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
    assert_eq!(part_1(input), 16);

    macro_rules! timeit {
        ($($body:tt)*) => {
            {
                let now = std::time::Instant::now();
                {
                    $($body)*
                }
                eprintln!("Took {:?}", now.elapsed());
            }
        };
    }

    timeit! {
        COUNT.store(0, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 1);
    }
    timeit! {
        COUNT.store(6, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 16);
    }
    timeit! {
        COUNT.store(10, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 50);
    }
    timeit! {
        COUNT.store(50, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 1594);
    }
    timeit! {
        COUNT.store(100, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 6536);
    }
    timeit! {
        COUNT.store(500, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 167004);
    }
    timeit! {
        COUNT.store(1000, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 668697);
    }
    timeit! {
        COUNT.store(5000, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(part_2(input), 16733044);
    }
}
