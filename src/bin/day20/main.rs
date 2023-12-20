#![allow(unused_imports)]
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ModuleKind {
    FlipFlop { state: bool },
    Conjunction { remembered_inputs: Vec<bool> },
    Broadcast,
    Button,
}

impl ModuleKind {
    fn apply_pulse(&mut self, incoming: bool, connection_idx: usize) -> Option<bool> {
        match self {
            ModuleKind::FlipFlop { state } => {
                match incoming {
                    // If a flipflop receives a high pulse, it is ignored and nothing happens.
                    true => None,
                    // If a flip-flop module receives a low pulse, it flips between on and off.
                    // If it was off, it turns on and sends a high pulse.
                    // If it was on, it turns off and sends a low pulse.
                    false => {
                        *state = !*state;
                        Some(*state)
                    }
                }
            }
            ModuleKind::Conjunction { remembered_inputs } => {
                remembered_inputs[connection_idx] = incoming;
                Some(remembered_inputs.iter().any(|&p| !p))
            }
            ModuleKind::Button | ModuleKind::Broadcast => {
                assert!(incoming == false);
                Some(false)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Module<'a> {
    kind: ModuleKind,
    /// (connection name, index of this input in connection's remembered inputs)
    connections: Vec<(&'a str, usize)>,
}
impl<'a> Module<'a> {
    fn apply_pulse(
        &mut self,
        incoming: bool,
        connection_idx: usize,
    ) -> Vec<(&'a str, usize, bool)> {
        if let Some(next_pulse) = self.kind.apply_pulse(incoming, connection_idx) {
            self.connections
                .iter()
                .map(|&(dst, idx)| (dst, idx, next_pulse))
                .collect()
        } else {
            vec![]
        }
    }
}

fn part_1<'input>(input: &'input str) -> usize {
    let mut modules: HashMap<&'input str, RefCell<Module<'input>>> = input
        .lines()
        .map(str::trim)
        .chain(["button -> broadcaster"])
        .map(|line| {
            let (module, connections) = line.split_once(" -> ").unwrap();
            let (module_kind, name) = match (module, module.chars().next().unwrap()) {
                (_, '%') => (ModuleKind::FlipFlop { state: false }, &module[1..]),
                (_, '&') => (
                    ModuleKind::Conjunction {
                        remembered_inputs: vec![],
                    },
                    &module[1..],
                ),
                ("broadcaster", _) => (ModuleKind::Broadcast, module),
                ("button", _) => (ModuleKind::Button, module),
                _ => unreachable!(),
            };
            (
                name,
                RefCell::new(Module {
                    kind: module_kind,
                    connections: connections
                        .split(',')
                        .map(str::trim)
                        .map(|module| (module, 0))
                        .collect_vec(),
                }),
            )
        })
        .collect();

    for (name, module) in &modules {
        let mut module = module.borrow_mut();
        for (connection, conn_idx) in module.connections.iter_mut() {
            let Some(dest) = modules.get(*connection) else {
                // connection is a module with no outputs
                continue;
            };
            let mut dest = dest.borrow_mut();
            if let ModuleKind::Conjunction { remembered_inputs } = &mut dest.kind {
                let idx = remembered_inputs.len();
                remembered_inputs.push(false);
                *conn_idx = idx;
            }
        }
    }

    let mut total_low_sent = 0;
    let mut total_high_sent = 0;

    for _ in 0..1000 {
        // (from, to, idx, is_high)
        let mut pulses: VecDeque<(&str, &str, usize, bool)> =
            VecDeque::from([("button", "broadcaster", 0, false)]);

        while let Some((src, dest, idx, high)) = pulses.pop_front() {
            // println!("{src} -{high}-> {dest}");
            if high {
                total_high_sent += 1;
            } else {
                total_low_sent += 1;
            }
            let Some(module) = modules.get(dest) else {
                // module with no outputs
                continue;
            };
            let mut module = module.borrow_mut();
            pulses.extend(
                module
                    .apply_pulse(high, idx)
                    .into_iter()
                    .map(|(a, b, c)| (dest, a, b, c)),
            );
        }
    }

    dbg!(total_high_sent, total_low_sent);

    total_high_sent * total_low_sent
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:20:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:20:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(part_1(input), 32000000);
    // assert_eq!(part_2(input), 42);
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(part_1(input), 11687500);
    // assert_eq!(part_2(input), 42);
}
