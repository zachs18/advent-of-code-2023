#![allow(unused_imports)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

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
    let h = data.len();
    let w = data[0].len();

    let y: usize = 0;
    let x: usize = 1;

    let goal_y: usize = h - 1;
    let goal_x: usize = w - 2;

    // (y,x,path_length,seen)
    let mut queue = VecDeque::from([(y, x, 0_usize, HashSet::from([(y, x)]))]);
    let mut longest = 0;
    while let Some((y, x, length, seen)) = queue.pop_front() {
        if (y, x) == (goal_y, goal_x) {
            longest = longest.max(length);
            continue;
        }

        if b"^.".contains(&data[y][x])
            && y > 0
            && !seen.contains(&(y - 1, x))
            && data[y - 1][x] != b'#'
        {
            let y = y - 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if b"v.".contains(&data[y][x])
            && y + 1 < h
            && !seen.contains(&(y + 1, x))
            && data[y + 1][x] != b'#'
        {
            let y = y + 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if b"<.".contains(&data[y][x])
            && x > 0
            && !seen.contains(&(y, x - 1))
            && data[y][x - 1] != b'#'
        {
            let x = x - 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
        if b">.".contains(&data[y][x])
            && x + 1 < w
            && !seen.contains(&(y, x + 1))
            && data[y][x + 1] != b'#'
        {
            let x = x + 1;
            let mut seen = seen.clone();
            seen.insert((y, x));
            queue.push_back((y, x, length + 1, seen))
        }
    }
    longest
}

#[derive(Debug, Clone)]
struct Graph {
    /// edge ID is index, value is (start node id, end node id, length)
    /// edges are bidirectional
    edges: HashMap<usize, (usize, usize, usize)>,
    /// node ID is index, value is the edges the node is connected to
    node_edges: HashMap<usize, Vec<usize>>,
    start_node_id: usize,
    end_node_id: usize,
}

impl Graph {
    fn new_naive(data: &[&[u8]]) -> Self {
        let h = data.len();
        let w = data[0].len();
        let mut edges = vec![];
        let mut nodes = vec![];
        let mut position_to_node: HashMap<(usize, usize), usize> = HashMap::new();
        macro_rules! register_node {
            ($y:expr, $x:expr) => {
                *position_to_node.entry(($y, $x)).or_insert_with(|| {
                    let id = nodes.len();
                    nodes.push(Vec::with_capacity(4));
                    id
                })
            };
        }
        macro_rules! new_edge {
            ($node1:expr, $node2:expr) => {{
                let node1 = $node1;
                let node2 = $node2;
                let edge_id = edges.len();
                edges.push((node1, node2, 1));
                nodes[node1].push(edge_id);
                nodes[node2].push(edge_id);
                edge_id
            }};
        }

        for y in 0..h - 1 {
            for x in 0..w - 1 {
                if data[y][x] == b'#' || (data[y + 1][x] == b'#' && data[y][x + 1] == b'#') {
                    continue;
                }
                let start = register_node!(y, x);
                if data[y + 1][x] != b'#' {
                    let end = register_node!(y + 1, x);
                    new_edge!(start, end);
                }
                if data[y][x + 1] != b'#' {
                    let end = register_node!(y, x + 1);
                    new_edge!(start, end);
                }
            }
        }

        Self {
            edges: edges.into_iter().enumerate().collect(),
            node_edges: nodes.into_iter().enumerate().collect(),
            start_node_id: position_to_node.get(&(0, 1)).copied().unwrap(),
            end_node_id: position_to_node.get(&(h - 1, w - 2)).copied().unwrap(),
        }
    }

    fn simplify(mut self) -> Self {
        'retry: loop {
            for (&node_id, node_edges) in self.node_edges.iter_mut() {
                if let [e1, e2] = **node_edges {
                    let (e1n1, e1n2, e1len) = self.edges[&e1];
                    let (e2n1, e2n2, e2len) = self.edges[&e2];
                    let length = e1len + e2len;
                    let n1 = if e1n1 != node_id { e1n1 } else { e1n2 };
                    let n2 = if e2n1 != node_id { e2n1 } else { e2n2 };
                    self.edges.insert(e1, (n1, n2, length));
                    self.edges.remove(&e2);
                    self.node_edges.remove(&node_id);

                    for edge_id in self.node_edges.get_mut(&n2).unwrap() {
                        if *edge_id == e2 {
                            *edge_id = e1;
                        }
                    }

                    continue 'retry;
                }
            }
            break;
        }

        self
    }
}

fn part_2(input: &str) -> usize {
    let data = input
        .lines()
        .map(str::trim)
        .map(str::as_bytes)
        .collect_vec();

    let graph = Graph::new_naive(&data).simplify();

    // (current node id, path length, seen node ids)
    let mut queue =
        VecDeque::from([(graph.start_node_id, 0, HashSet::from([graph.start_node_id]))]);
    let mut longest = 0;
    while let Some((node, length, seen)) = queue.pop_front() {
        if node == graph.end_node_id {
            longest = longest.max(length);
            continue;
        }

        for &edge in &graph.node_edges[&node] {
            let edge = graph.edges[&edge];
            let n2 = if node != edge.0 { edge.0 } else { edge.1 };
            if seen.contains(&n2) {
                continue;
            }
            let mut seen = seen.clone();
            seen.insert(n2);

            queue.push_back((n2, length + edge.2, seen));
        }
    }
    longest
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:23:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:23:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "\
#.###
#...#
###.#";
    assert_eq!(part_1(input), 4);
    assert_eq!(part_2(input), 4);

    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    assert_eq!(part_1(input), 94);
    assert_eq!(part_2(input), 154);
}
