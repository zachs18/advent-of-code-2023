use std::collections::{HashMap, HashSet};

use aoc_2023::*;
use aoc_driver::*;
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get_rating(&self, rating: Rating) -> usize {
        match rating {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }

    fn sum_ratings(&self) -> usize {
        let Part { x, m, a, s } = *self;
        x + m + a + s
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

enum ConditionKind {
    LessThan(usize),
    GreaterThan(usize),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

enum Rating {
    X,
    M,
    A,
    S,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

enum Destination<'a> {
    Workflow(&'a str),
    Accept,
    Reject,
}

impl<'a> Destination<'a> {
    fn as_accepted(&self) -> Result<bool, &'a str> {
        match self {
            Destination::Workflow(dest) => Err(dest),
            Destination::Accept => Ok(true),
            Destination::Reject => Ok(false),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Condition<'a> {
    rating: Rating,
    condition: ConditionKind,
    destination: Destination<'a>,
}

impl<'a> Condition<'a> {
    fn run(&self, part: Part) -> Option<Destination<'a>> {
        let value = part.get_rating(self.rating);
        let applies = match self.condition {
            ConditionKind::LessThan(a) => value < a,
            ConditionKind::GreaterThan(a) => value > a,
        };
        applies.then_some(self.destination)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Workflow<'a> {
    conditions: Vec<Condition<'a>>,
    otherwise: Destination<'a>,
}

impl<'a> Workflow<'a> {
    fn run(&self, part: Part) -> Result<bool, &'a str> {
        for &condition in &self.conditions {
            if let Some(dest) = condition.run(part) {
                return dest.as_accepted();
            }
        }
        self.otherwise.as_accepted()
    }
}

fn make_dest(s: &str) -> Destination<'_> {
    match s {
        "A" => Destination::Accept,
        "R" => Destination::Reject,
        _ => Destination::Workflow(s),
    }
}

fn part_1(input: &str) -> usize {
    let data = input.lines().map(str::trim).collect_vec();
    let [workflows, ratings]: [&[&str]; 2] = data
        .split(|line| line.is_empty())
        .collect_vec()
        .try_into()
        .unwrap();
    let parts = ratings
        .iter()
        .map(|line| {
            let line = line.strip_prefix("{x=").unwrap();
            let (x, line) = line.split_once(",m=").unwrap();
            let (m, line) = line.split_once(",a=").unwrap();
            let (a, line) = line.split_once(",s=").unwrap();
            let s = line.strip_suffix("}").unwrap();
            Part {
                x: x.parse().unwrap(),
                m: m.parse().unwrap(),
                a: a.parse().unwrap(),
                s: s.parse().unwrap(),
            }
        })
        .collect_vec();
    let workflows: HashMap<&str, Workflow<'_>> = workflows
        .iter()
        .map(|line| {
            let (workflow, line) = line.split_once('{').unwrap();
            let line = line.strip_suffix('}').unwrap();
            let mut conditions = vec![];
            let mut default = None;
            for condition in line.split(',') {
                let Some((comparison, destination)) = condition.split_once(':') else {
                    default = Some(make_dest(condition));
                    break;
                };
                let rating = comparison.as_bytes()[0];
                let number: usize = comparison[2..].parse().unwrap();
                let comparison = comparison.as_bytes()[1];
                let rating = match rating {
                    b'x' => Rating::X,
                    b'm' => Rating::M,
                    b'a' => Rating::A,
                    b's' => Rating::S,
                    _ => unreachable!(),
                };
                let comparison = match comparison {
                    b'<' => ConditionKind::LessThan(number),
                    b'>' => ConditionKind::GreaterThan(number),
                    _ => unreachable!(),
                };
                conditions.push(Condition {
                    rating,
                    condition: comparison,
                    destination: make_dest(destination),
                });
            }
            (
                workflow,
                Workflow {
                    conditions,
                    otherwise: default.unwrap(),
                },
            )
        })
        .collect();

    let mut total_rating_of_accepted = 0;
    for part in parts {
        let mut seen: HashSet<&Workflow<'_>> = HashSet::new();
        let mut workflow = workflows.get("in").unwrap();
        loop {
            if !seen.insert(workflow) {
                // loop
                break;
            }
            // dbg!(workflow, part);
            match workflow.run(part) {
                Ok(is_accepted) => {
                    if is_accepted {
                        total_rating_of_accepted += part.sum_ratings();
                    }
                    continue;
                }
                Err(dest) => workflow = workflows.get(dest).unwrap(),
            }
        }
    }
    total_rating_of_accepted
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:19:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:19:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(part_1(input), 42);
    // assert_eq!(part_2(input), 42);
}
