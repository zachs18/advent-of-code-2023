use aoc_2023::PreParsed;
use aoc_driver::*;
use glam::{DVec3, I64Vec3};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: DVec3,
    velocity: DVec3,
}

#[derive(Debug, Clone, Copy)]
struct Hailstone2 {
    position: I64Vec3,
    velocity: I64Vec3,
}

impl Hailstone2 {
    fn t(self, n: i64) -> I64Vec3 {
        self.position + n * self.velocity
    }

    fn as_solution(self) -> i64 {
        let I64Vec3 { x, y, z } = self.position;
        x + y + z
    }
}

fn parse(input: &str) -> Vec<(Hailstone, Hailstone2)> {
    input
        .lines()
        .map(str::trim)
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let mut pos = pos.split(',').map(|s| s.trim().parse().unwrap());
            let mut vel = vel.split(',').map(|s| s.trim().parse().unwrap());
            let pos = I64Vec3 {
                x: pos.next().unwrap(),
                y: pos.next().unwrap(),
                z: pos.next().unwrap(),
            };
            let vel = I64Vec3 {
                x: vel.next().unwrap(),
                y: vel.next().unwrap(),
                z: vel.next().unwrap(),
            };
            (
                Hailstone {
                    position: pos.as_dvec3(),
                    velocity: vel.as_dvec3(),
                },
                Hailstone2 {
                    position: pos,
                    velocity: vel,
                },
            )
        })
        .collect_vec()
}

fn intersect_within_test_area(
    h1: Hailstone,
    h2: Hailstone,
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
) -> bool {
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
    // this is a system of two linear equations with two unknowns so *should* be solvable
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

fn part_1(data: &Vec<(Hailstone, Hailstone2)>) -> usize {
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
        let stone1 = data[i].0;
        for j in i + 1..data.len() {
            let stone2 = data[j].0;
            if intersect_within_test_area(stone1, stone2, min, min, max, max) {
                intersection_count += 1;
            }
        }
    }

    intersection_count
}

fn intersect2(h1: Hailstone2, h2: Hailstone2) -> bool {
    let Hailstone2 {
        position: I64Vec3 {
            x: x0_1,
            y: y0_1,
            z: z0_1,
        },
        velocity: I64Vec3 {
            x: vx_1,
            y: vy_1,
            z: vz_1,
        },
    } = h1;

    let Hailstone2 {
        position: I64Vec3 {
            x: x0_2,
            y: y0_2,
            z: z0_2,
        },
        velocity: I64Vec3 {
            x: vx_2,
            y: vy_2,
            z: vz_2,
        },
    } = h2;

    // None means IDK,
    // Some(Err) means impossible
    // Some(Ok(t)) means if it is possible it happens at time t
    let check = |p0_1, v1, p0_2, v2| -> Option<Result<i64, ()>> {
        let num = p0_2 - p0_1;
        let denom = v1 - v2;
        match (num, denom) {
            (0, 0) => None,
            (_, 0) => Some(Err(())),
            _ => Some(Ok(num / denom)),
        }
    };

    let t = check(x0_1, vx_1, x0_2, vx_2)
        .or_else(|| check(y0_1, vy_1, y0_2, vy_2))
        .or_else(|| check(z0_1, vz_1, z0_2, vz_2))
        .unwrap_or(Ok(0));

    match t {
        Ok(t) => h1.t(t) == h2.t(t),
        Err(_) => false,
    }
}

// we could consider part 2 as a system of 900 equations (x,y,z for each hailstone)
// in 306 unknowns (t for collision with each hailstone, plus x,y,z position  and x,y,z velocity of the rock)
// rx0 + t0*rvx = h0x0 + t0*h0vx
// ry0 + t0*rvy = h0y0 + t0*h0vy
// rz0 + t0*rvz = h0z0 + t0*h0vz
// rx0 + t1*rvx = h1x0 + t1*h1vx
// ry0 + t1*rvy = h1y0 + t1*h1vy
// rz0 + t1*rvz = h1z0 + t1*h1vz
// etc
// in general, if we consider n hailstones, we have a system of 3n equations to find n+6 unknowns,
// which is solvable if 3n>=n+6, i.e. n=3.
// ~~assuming that the input is actually solvable, it shouldn't matter which 3 hailstones we choose to solve with.~~
// actually that's not true, e.g. if the three hailstones we choose have the same velocity, then there are multiple solutions.
// Also, this is not a system of *linear* equations, since we are multiplying some of the variables together,
// so the "it should be solvable" doesn't necessarily apply.

#[cfg(not(feature = "day24part2"))]
fn try_solve2(h0: Hailstone2, h1: Hailstone2, h2: Hailstone2) -> Option<Hailstone2> {
    panic!("my day 24 part 2 requires z3")
}

#[cfg(feature = "day24part2")]
fn try_solve2(h0: Hailstone2, h1: Hailstone2, h2: Hailstone2) -> Option<Hailstone2> {
    // unknowns: rx0, ry0, rz0, rvx, rvy, rvz, t0, t1, t2
    // equations:
    //  A: rx0 + t0*rvx = h0x0 + t0*h0vx
    //  B: ry0 + t0*rvy = h0y0 + t0*h0vy
    //  C: rz0 + t0*rvz = h0z0 + t0*h0vz
    //  D: rx0 + t1*rvx = h1x0 + t1*h1vx
    //  E: ry0 + t1*rvy = h1y0 + t1*h1vy
    //  F: rz0 + t1*rvz = h1z0 + t1*h1vz
    //  G: rx0 + t2*rvx = h2x0 + t2*h2vx
    //  H: ry0 + t2*rvy = h2y0 + t2*h2vy
    //  I: rz0 + t2*rvz = h2z0 + t2*h2vz

    let Hailstone2 {
        position: I64Vec3 {
            x: h0x0,
            y: h0y0,
            z: h0z0,
        },
        velocity: I64Vec3 {
            x: h0vx,
            y: h0vy,
            z: h0vz,
        },
    } = h0;

    let Hailstone2 {
        position: I64Vec3 {
            x: h1x0,
            y: h1y0,
            z: h1z0,
        },
        velocity: I64Vec3 {
            x: h1vx,
            y: h1vy,
            z: h1vz,
        },
    } = h1;

    let Hailstone2 {
        position: I64Vec3 {
            x: h2x0,
            y: h2y0,
            z: h2z0,
        },
        velocity: I64Vec3 {
            x: h2vx,
            y: h2vy,
            z: h2vz,
        },
    } = h2;

    use z3::{ast::Int, Config, Context};
    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    let ctx = Context::new(&cfg);

    macro_rules! vars {
        ($($name:ident),* $(,)?) => {
            $(
                let $name = Int::new_const(&ctx, stringify!($name));
            )*
        };
    }

    vars! {
        rx0, ry0, rz0,
        rvx, rvy, rvz,
        t1, t2, t0,
    }

    macro_rules! consts {
        ($($name:ident),* $(,)?) => {
            $(
                let $name = Int::from_i64(&ctx, $name);
            )*
        };
    }

    consts! {
        h0x0, h0y0, h0z0,
        h0vx, h0vy, h0vz,
        h1x0, h1y0, h1z0,
        h1vx, h1vy, h1vz,
        h2x0, h2y0, h2z0,
        h2vx, h2vy, h2vz,
    }

    //  A: rx0 + t0*rvx = h0x0 + t0*h0vx
    //  B: ry0 + t0*rvy = h0y0 + t0*h0vy
    //  C: rz0 + t0*rvz = h0z0 + t0*h0vz
    //  D: rx0 + t1*rvx = h1x0 + t1*h1vx
    //  E: ry0 + t1*rvy = h1y0 + t1*h1vy
    //  F: rz0 + t1*rvz = h1z0 + t1*h1vz
    //  G: rx0 + t2*rvx = h2x0 + t2*h2vx
    //  H: ry0 + t2*rvy = h2y0 + t2*h2vy
    //  I: rz0 + t2*rvz = h2z0 + t2*h2vz
    let solver = z3::Solver::new(&ctx);

    macro_rules! equations {
        () => {};
        ($r0:ident + $t:ident * $rv:ident = $h0:ident + $_t:ident * $hv:ident; $($rest:tt)*) => {
            let lhs_product = Int::mul(&ctx, &[&$t, &$rv]);
            let lhs = Int::add(&ctx, &[&$r0, &lhs_product]);
            let rhs_product = Int::mul(&ctx, &[&$t, &$hv]);
            let rhs = Int::add(&ctx, &[&$h0, &rhs_product]);
            solver.assert(&<Int as z3::ast::Ast>::_eq(&lhs, &rhs));
            equations!($($rest)*);
        };
    }

    equations! {
        rx0 + t0*rvx = h0x0 + t0*h0vx;
        ry0 + t0*rvy = h0y0 + t0*h0vy;
        rz0 + t0*rvz = h0z0 + t0*h0vz;
        rx0 + t1*rvx = h1x0 + t1*h1vx;
        ry0 + t1*rvy = h1y0 + t1*h1vy;
        rz0 + t1*rvz = h1z0 + t1*h1vz;
        rx0 + t2*rvx = h2x0 + t2*h2vx;
        ry0 + t2*rvy = h2y0 + t2*h2vy;
        rz0 + t2*rvz = h2z0 + t2*h2vz;
    }

    let zero = Int::from_u64(&ctx, 0);
    macro_rules! constraints {
        ($($t:ident),*) => {
            $(
                solver.assert(&Int::ge(&$t, &zero));
            )*
        };
    }
    constraints!(t0, t1, t2);

    let result = solver.check();

    match result {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => todo!(),
        z3::SatResult::Sat => {
            let model = solver.get_model()?;
            macro_rules! get_values {
                ($($value:ident),* $(,)?) => {
                    $(
                        let $value = model.eval(&$value, true).unwrap().as_i64().unwrap();
                    )*
                };
            }
            get_values! {
                rx0, ry0, rz0,
                rvx, rvy, rvz,
            }
            Some(Hailstone2 {
                position: I64Vec3 {
                    x: rx0,
                    y: ry0,
                    z: rz0,
                },
                velocity: I64Vec3 {
                    x: rvx,
                    y: rvy,
                    z: rvz,
                },
            })
        }
    }
}

fn part_2(data: &Vec<(Hailstone, Hailstone2)>) -> i64 {
    for i in 0..data.len() {
        let h0 = data[i].1;
        for j in i + 1..data.len() {
            let h1 = data[j].1;
            for k in j + 1..data.len() {
                let h2 = data[k].1;
                if let Some(rock) = try_solve2(h0, h1, h2) {
                    if data.iter().all(|&(_, h)| intersect2(rock, h)) {
                        return rock.as_solution();
                    } else {
                        dbg!(h0, h1, h2, rock, "failed but not actually?");
                    }
                }
            }
        }
    }

    todo!("that didn't work")
}

fn main() {
    let session = std::fs::read_to_string(".session.txt").unwrap();
    let session = session.trim();
    let mut both = PreParsed::new(parse, part_1, part_2);
    let part_2 = both.part_2();
    if let Err(error) = aoc_magic!(session, 2023:24:2, part_2) {
        eprintln!("Part 2 failed: {error:?}");
    }
    let part_1 = both.part_1();
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
    let data = parse(input);
    assert_eq!(part_1(&data), 2);
    assert_eq!(part_2(&data), 47);
}
