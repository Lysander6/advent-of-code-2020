use std::str::FromStr;

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr, PartialEq)]
enum Move {
    #[display("N{0}")]
    North(i32),
    #[display("S{0}")]
    South(i32),
    #[display("E{0}")]
    East(i32),
    #[display("W{0}")]
    West(i32),
    #[display("L{0}")]
    LeftTurn(i32),
    #[display("R{0}")]
    RightTurn(i32),
    #[display("F{0}")]
    Forward(i32),
}

#[derive(Debug)]
pub struct Problem {
    moves: Vec<Move>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem {
            moves: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> i32 {
    let Problem { moves } = p;
    let mut facing_direction = (1, 0);
    let mut x = 0i32;
    let mut y = 0i32;

    for m in moves {
        match &m {
            Move::North(d) => y += d,
            Move::South(d) => y -= d,
            Move::East(d) => x += d,
            Move::West(d) => x -= d,
            Move::LeftTurn(d) => {
                facing_direction = match d {
                    90 => (-facing_direction.1, facing_direction.0),
                    180 => (-facing_direction.0, -facing_direction.1),
                    270 => (facing_direction.1, -facing_direction.0),
                    360 => facing_direction,
                    _ => unreachable!(),
                };
            }
            Move::RightTurn(d) => {
                facing_direction = match d {
                    90 => (facing_direction.1, -facing_direction.0),
                    180 => (-facing_direction.0, -facing_direction.1),
                    270 => (-facing_direction.1, facing_direction.0),
                    360 => facing_direction,
                    _ => unreachable!(),
                }
            }
            Move::Forward(d) => {
                x += d * facing_direction.0;
                y += d * facing_direction.1;
            }
        }
    }

    x.abs() + y.abs()
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> i32 {
    let Problem { moves } = p;
    let mut waypoint_x = 10i32;
    let mut waypoint_y = 1i32;
    let mut x = 0i32;
    let mut y = 0i32;

    for m in moves {
        match &m {
            Move::North(d) => waypoint_y += d,
            Move::South(d) => waypoint_y -= d,
            Move::East(d) => waypoint_x += d,
            Move::West(d) => waypoint_x -= d,
            Move::LeftTurn(d) => {
                (waypoint_x, waypoint_y) = match d {
                    90 => (-waypoint_y, waypoint_x),
                    180 => (-waypoint_x, -waypoint_y),
                    270 => (waypoint_y, -waypoint_x),
                    360 => (waypoint_x, waypoint_y),
                    _ => unreachable!(),
                };
            }
            Move::RightTurn(d) => {
                (waypoint_x, waypoint_y) = match d {
                    90 => (waypoint_y, -waypoint_x),
                    180 => (-waypoint_x, -waypoint_y),
                    270 => (-waypoint_y, waypoint_x),
                    360 => (waypoint_x, waypoint_y),
                    _ => unreachable!(),
                };
            }
            Move::Forward(d) => {
                x += d * waypoint_x;
                y += d * waypoint_y;
            }
        }
    }

    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p), 25);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_2(&p), 286);
    }
}
