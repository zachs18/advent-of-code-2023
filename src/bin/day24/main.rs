#![allow(unused_imports)]
use std::ops::RangeInclusive;

use aoc_2023::*;
use aoc_driver::*;
use glam::{DVec3, I64Vec3};
use itertools::Itertools;
use zachs18_stdx::*;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: DVec3,
    velocity: DVec3,
}

impl Hailstone {
    /// (pos, time)
    fn x0(mut self) -> (Self, f64) {
        if self.velocity.x == 0.0 {
            unimplemented!("not in my input")
        }
        let t = -self.position.x / self.velocity.x;
        self.position += self.velocity * t;
        (self, t)
    }
}

fn intersect_within_first_quadrant(
    h1: Hailstone,
    h2: Hailstone,
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
) -> bool {
    // if (h1.position.x < 0.0 && h1.velocity.x <= 0.0)
    //     || (h2.position.x < 0.0 && h2.velocity.x <= 0.0)
    //     || (h1.position.x > x_max && h1.velocity.x >= 0.0)
    //     || (h2.position.x > x_max && h2.velocity.x >= 0.0)
    //     || (h1.position.y < 0.0 && h1.velocity.y <= 0.0)
    //     || (h2.position.y < 0.0 && h2.velocity.y <= 0.0)
    //     || (h1.position.y > y_max && h1.velocity.y >= 0.0)
    //     || (h2.position.y > y_max && h2.velocity.y >= 0.0)
    // {
    //     return false;
    // }

    let Hailstone {
        position: DVec3 {
            x: x0_1,
            y: y0_1,
            z: _,
        },
        velocity: DVec3 {
            x: vx_1,
            y: vy_1,
            z: _,
        },
    } = h1;

    let Hailstone {
        position: DVec3 {
            x: x0_2,
            y: y0_2,
            z: _,
        },
        velocity: DVec3 {
            x: vx_2,
            y: vy_2,
            z: _,
        },
    } = h2;

    // A: y0_1 + t1*vy_1 = y0_2 + t2*vy_2
    // B: x0_1 + t1*vx_1 = x0_2 + t2*vx_2
    // this is a system of two equations with two unknowns so *should* be solvable
    // A - (vy_1/vx_1)*B: y0_1 + t1*vy_1 - (vy_1/vx_1)*(x0_1 + t1*vx_1) = y0_2 + t2*vy_2 - (vy_1/vx_1)*(x0_2 + t2*vx_2)
    //      y0_1 + t1*vy_1 - (vy_1/vx_1)*x0_1 - t1*vy_1 = y0_2 + t2*vy_2 - (vy_1/vx_1)*(x0_2 + t2*vx_2)
    //      y0_1 - (vy_1/vx_1)*x0_1 = y0_2 + t2*vy_2 - (vy_1/vx_1)*(x0_2 + t2*vx_2)
    //      y0_1 - (vy_1/vx_1)*x0_1 = y0_2 + t2*vy_2 - (vy_1/vx_1)*x0_2 - (vy_1/vx_1)*t2*vx_2
    //      y0_1 - (vy_1/vx_1)*x0_1 = y0_2 + t2*(vy_2 - (vy_1/vx_1)*vx_2) - (vy_1/vx_1)*x0_2
    //      y0_2 + t2*(vy_2 - (vy_1/vx_1)*vx_2) - (vy_1/vx_1)*x0_2 = y0_1 - (vy_1/vx_1)*x0_1
    //      t2*(vy_2 - (vy_1/vx_1)*vx_2) = - y0_2 + (vy_1/vx_1)*x0_2 + y0_1 - (vy_1/vx_1)*x0_1
    //      t2*(vy_2 - (vy_1/vx_1)*vx_2) = (y0_1 - y0_2) + (vy_1/vx_1)*(x0_2 - x0_1)
    //      t2 = ((y0_1 - y0_2) + (vy_1/vx_1)*(x0_2 - x0_1)) / (vy_2 - (vy_1/vx_1)*vx_2)
    // B: x0_1 + t1*vx_1 = x0_2 + t2*vx_2
    //      t1*vx_1 = x0_2 - x0_1 + t2*vx_2
    //      t1 = (x0_2 - x0_1 + t2*vx_2) / vx_1

    let t2 = ((y0_1 - y0_2) + (vy_1 / vx_1) * (x0_2 - x0_1)) / (vy_2 - (vy_1 / vx_1) * vx_2);
    let t1 = (x0_2 - x0_1 + t2 * vx_2) / vx_1;

    let y = y0_1 + t1 * vy_1;
    let x = x0_1 + t1 * vx_1;

    t1 >= 0.0 && t2 >= 0.0 && x >= x_min && x <= x_max && y >= y_min && y <= y_max
}

fn part_1(input: &str) -> usize {
    let mut data = input
        .lines()
        .map(str::trim)
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let mut pos = pos.split(',').map(|s| s.trim().parse().unwrap());
            let mut vel = vel.split(',').map(|s| s.trim().parse().unwrap());
            let pos = DVec3 {
                x: pos.next().unwrap(),
                y: pos.next().unwrap(),
                z: pos.next().unwrap(),
            };
            let vel = DVec3 {
                x: vel.next().unwrap(),
                y: vel.next().unwrap(),
                z: vel.next().unwrap(),
            };
            Hailstone {
                position: pos,
                velocity: vel,
            }
        })
        .collect_vec();

    // // the test area is [200000000000000, 400000000000000] for x and y,
    // // so just subtract 200000000000000 from all the x and y values and then
    // // use [0, 200000000000000] as the test area.

    // for stone in &mut data {
    //     stone.position.x -= 200000000000000.0;
    //     stone.position.y -= 200000000000000.0;
    // }

    let mut intersection_count = 0;

    #[cfg(not(test))]
    let min = 200000000000000.0;
    #[cfg(not(test))]
    let max = 400000000000000.0;
    #[cfg(test)]
    let min = 7.0;
    #[cfg(test)]
    let max = 27.0;

    for i in 0..data.len() {
        let stone1 = data[i];
        for j in i + 1..data.len() {
            let stone2 = data[j];
            if intersect_within_first_quadrant(stone1, stone2, min, min, max, max) {
                intersection_count += 1;
            }
        }
    }

    intersection_count
}

fn part_2(input: &str) -> usize {
    todo!()
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    if let Err(error) = aoc_magic!(session, 2023:24:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    if let Err(error) = aoc_magic!(session, 2023:24:1, part_1) {
        eprintln!("Part 1 failed: {error:?}");
    }
}

#[test]
fn example() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    assert_eq!(part_1(input), 42);
    assert_eq!(part_2(input), 42);
}
