use std::str::FromStr;

use anyhow::bail;
use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq)]
enum FacingDirection {
    North = 0isize,
    East = 1,
    South = 2,
    West = 3,
}

impl TryFrom<i32> for FacingDirection {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FacingDirection::North),
            1 => Ok(FacingDirection::East),
            2 => Ok(FacingDirection::South),
            3 => Ok(FacingDirection::West),
            _ => bail!("Invalid facing direction"),
        }
    }
}

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

/// # Errors
///
/// Returns an error if the `rotation` isn't either `Move::LeftTurn` or
/// `Move::RightTurn`.
fn rotate_facing_direction(
    facing_direction: FacingDirection,
    rotation: &Move,
) -> Result<FacingDirection, anyhow::Error> {
    let (direction_sign, degrees) = match rotation {
        Move::LeftTurn(d) => (-1, d),
        Move::RightTurn(d) => (1, d),
        _ => bail!("not a rotation"),
    };

    let mut facing_direction = facing_direction as i32;
    facing_direction += direction_sign * degrees / 90;
    facing_direction %= 4;

    if facing_direction < 0 {
        facing_direction += 4;
    }

    facing_direction.try_into()
}

/// # Errors
///
/// See `rotate_facing_direction`.
pub fn solve_part_1(p: &Problem) -> Result<i32, anyhow::Error> {
    let Problem { moves } = p;
    let mut facing_direction = FacingDirection::East;
    let mut x = 0i32;
    let mut y = 0i32;

    for m in moves {
        match m {
            Move::North(d) => y += *d,
            Move::South(d) => y -= *d,
            Move::East(d) => x += *d,
            Move::West(d) => x -= *d,
            Move::LeftTurn(_) | Move::RightTurn(_) => {
                facing_direction = rotate_facing_direction(facing_direction, m)?;
            }
            Move::Forward(d) => match facing_direction {
                FacingDirection::North => y += *d,
                FacingDirection::South => y -= *d,
                FacingDirection::East => x += *d,
                FacingDirection::West => x -= *d,
            },
        }
    }

    Ok(x.abs() + y.abs())
}

/// # Errors
///
/// Returns error when `Move::LeftTurn` or `Move::RightTurn` has degress value
/// other than 90, 180, 270 or 360.
pub fn solve_part_2(p: &Problem) -> Result<i32, anyhow::Error> {
    let Problem { moves } = p;
    let mut waypoint_x = 10i32;
    let mut waypoint_y = 1i32;
    let mut x = 0i32;
    let mut y = 0i32;

    for m in moves {
        match m {
            Move::North(d) => {
                waypoint_y += *d;
            }
            Move::South(d) => {
                waypoint_y -= *d;
            }
            Move::East(d) => {
                waypoint_x += *d;
            }
            Move::West(d) => {
                waypoint_x -= *d;
            }
            Move::LeftTurn(d) => {
                (waypoint_x, waypoint_y) = match d {
                    90 => (-waypoint_y, waypoint_x),
                    180 => (-waypoint_x, -waypoint_y),
                    270 => (waypoint_y, -waypoint_x),
                    360 => (waypoint_x, waypoint_y),
                    _ => bail!("invalid rotation"),
                };
            }
            Move::RightTurn(d) => {
                (waypoint_x, waypoint_y) = match d {
                    90 => (waypoint_y, -waypoint_x),
                    180 => (-waypoint_x, -waypoint_y),
                    270 => (-waypoint_y, waypoint_x),
                    360 => (waypoint_x, waypoint_y),
                    _ => bail!("invalid rotation"),
                };
            }
            Move::Forward(d) => {
                x += *d * waypoint_x;
                y += *d * waypoint_y;
            }
        }
    }

    Ok(x.abs() + y.abs())
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
    fn test_rotate_facing_direction() {
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::LeftTurn(90)).unwrap(),
            FacingDirection::West
        );
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::LeftTurn(180)).unwrap(),
            FacingDirection::South
        );
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::LeftTurn(270)).unwrap(),
            FacingDirection::East
        );
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::LeftTurn(360)).unwrap(),
            FacingDirection::North
        );
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::RightTurn(90)).unwrap(),
            FacingDirection::East
        );
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::RightTurn(180)).unwrap(),
            FacingDirection::South
        );
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::RightTurn(270)).unwrap(),
            FacingDirection::West
        );
        assert_eq!(
            rotate_facing_direction(FacingDirection::North, &Move::RightTurn(360)).unwrap(),
            FacingDirection::North
        );
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(&p).unwrap(), 25);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_2(&p).unwrap(), 286);
    }
}
