use std::{
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

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
struct RangePart {
    /// Inclusive range (start, end)
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl RangePart {
    fn get_rating_range(&self, rating: Rating) -> RangeInclusive<usize> {
        let (min, max) = match rating {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        };
        min..=max
    }

    fn get_rating(&self, rating: Rating) -> (usize, usize) {
        match rating {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }

    fn set_rating(mut self, rating: Rating, new: (usize, usize)) -> Self {
        match rating {
            Rating::X => self.x = new,
            Rating::M => self.m = new,
            Rating::A => self.a = new,
            Rating::S => self.s = new,
        }
        self
    }

    fn sum_ratings(&self) -> usize {
        let RangePart {
            x: (x_min, x_max),
            m: (m_min, m_max),
            a: (a_min, a_max),
            s: (s_min, s_max),
        } = *self;
        let x_count = x_max - x_min + 1;
        let m_count = m_max - m_min + 1;
        let a_count = a_max - a_min + 1;
        let s_count = s_max - s_min + 1;
        let x_sum = x_max * (x_max + 1) / 2 - x_min * (x_min - 1) / 2;
        let m_sum = m_max * (m_max + 1) / 2 - m_min * (m_min - 1) / 2;
        let a_sum = a_max * (a_max + 1) / 2 - a_min * (a_min - 1) / 2;
        let s_sum = s_max * (s_max + 1) / 2 - s_min * (s_min - 1) / 2;

        return x_sum * m_count * a_count * s_count
            + x_count * m_sum * a_count * s_count
            + x_count * m_count * a_sum * s_count
            + x_count * m_count * a_count * s_sum;
    }

    fn full() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn distinct_ratings(&self) -> usize {
        let RangePart {
            x: (x_min, x_max),
            m: (m_min, m_max),
            a: (a_min, a_max),
            s: (s_min, s_max),
        } = *self;
        let x_count = x_max - x_min + 1;
        let m_count = m_max - m_min + 1;
        let a_count = a_max - a_min + 1;
        let s_count = s_max - s_min + 1;
        x_count * m_count * a_count * s_count
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

    fn run_range(&self, part: RangePart) -> Vec<(RangePart, Option<Destination<'a>>)> {
        let (min, max) = part.get_rating(self.rating);
        let (applies, doesnt) = match self.condition {
            ConditionKind::LessThan(a) => {
                if max < a {
                    (Some(part), None)
                } else if min >= a {
                    (None, Some(part))
                } else {
                    let lowmax = a - 1;
                    let highmin = a;
                    let applies = part.set_rating(self.rating, (min, lowmax));
                    let doesnt = part.set_rating(self.rating, (highmin, max));
                    (Some(applies), Some(doesnt))
                }
            }
            ConditionKind::GreaterThan(a) => {
                if min > a {
                    (Some(part), None)
                } else if max <= a {
                    (None, Some(part))
                } else {
                    let lowmax = a;
                    let highmin = a + 1;
                    let doesnt = part.set_rating(self.rating, (min, lowmax));
                    let applies = part.set_rating(self.rating, (highmin, max));
                    (Some(applies), Some(doesnt))
                }
            }
        };
        match (applies, doesnt) {
            (None, None) => unreachable!(),
            (None, Some(doesnt)) => vec![(doesnt, None)],
            (Some(applies), None) => vec![(applies, Some(self.destination))],
            (Some(applies), Some(doesnt)) => {
                vec![(applies, Some(self.destination)), (doesnt, None)]
            }
        }
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

    fn run_range(&self, part: RangePart) -> Vec<(RangePart, Destination<'a>)> {
        let mut not_done = vec![part];
        let mut done = vec![];
        for &condition in &self.conditions {
            let mut new_not_done = vec![];
            for part in not_done {
                for (part, dest) in condition.run_range(part) {
                    match dest {
                        Some(dest) => done.push((part, dest)),
                        None => new_not_done.push(part),
                    }
                }
            }
            not_done = new_not_done;
        }
        done.extend(not_done.into_iter().map(|part| (part, self.otherwise)));
        done
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
            let s = line.strip_suffix('}').unwrap();
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
    'parts: for part in parts {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            match workflow.run(part) {
                Ok(is_accepted) => {
                    if is_accepted {
                        total_rating_of_accepted += part.sum_ratings();
                    }
                    continue 'parts;
                }
                Err(dest) => workflow = workflows.get(dest).unwrap(),
            }
        }
    }
    total_rating_of_accepted
}

fn part_2(input: &str) -> usize {
    let data = input.lines().map(str::trim).collect_vec();
    let [workflows, ratings]: [&[&str]; 2] = data
        .split(|line| line.is_empty())
        .collect_vec()
        .try_into()
        .unwrap();
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

    let mut queue = VecDeque::from([(RangePart::full(), Destination::Workflow("in"))]);

    let mut total_number_of_accepted = 0;
    'parts: while let Some((part, at)) = queue.pop_front() {
        let workflow = match at {
            Destination::Workflow(at) => workflows.get(at).unwrap(),
            Destination::Accept => {
                total_number_of_accepted += part.distinct_ratings();
                continue 'parts;
            }
            Destination::Reject => continue 'parts,
        };
        queue.extend(dbg!(workflow.run_range(part)));
    }
    total_number_of_accepted
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
    assert_eq!(part_1(input), 19114);
    assert_eq!(part_2(input), 167409079868000);
}

#[test]
fn sum_ratings() {
    let p = RangePart {
        x: (1, 4),
        m: (1, 4),
        a: (1, 4),
        s: (1, 4),
    };
    dbg!(p.sum_ratings());
    panic!();
}
