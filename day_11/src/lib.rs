#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
use std::{collections::HashMap, str::FromStr};

use anyhow::bail;

#[derive(Clone, Debug, PartialEq)]
pub enum Space {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone, Debug)]
pub struct Problem {
    map: Vec<Vec<Space>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Ok(Space::Floor),
                        'L' => Ok(Space::Empty),
                        '#' => Ok(Space::Occupied),
                        _ => bail!("unknown character"),
                    })
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;

        Ok(Problem { map })
    }
}

fn neighbour_offsets(i: usize, j: usize, max_i: usize, max_j: usize) -> &'static [(isize, isize)] {
    match (i, j) {
        (0, 0) => &[(0, 1), (1, 0), (1, 1)],
        (0, j) if j == max_j => &[(0, -1), (1, 0), (1, -1)],
        (0, _) => &[(0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
        (i, 0) if i == max_i => &[(-1, 0), (-1, 1), (0, 1)],
        (_, 0) => &[(-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)],
        (i, j) if i == max_i && j == max_j => &[(0, -1), (-1, 0), (-1, -1)],
        (i, _) if i == max_i => &[(0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1)],
        (_, j) if j == max_j => &[(0, -1), (-1, 0), (1, 0), (-1, -1), (1, -1)],
        (_, _) => &[
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ],
    }
}

fn run<F>(
    input: &[Vec<Space>],
    occupied_seats_limit: usize,
    occupied_seats_count_fn: F,
) -> (Vec<Vec<Space>>, bool)
where
    F: Fn(usize, usize, usize, usize) -> usize,
{
    let max_i = input.len() - 1;
    let max_j = input[0].len() - 1;

    let mut output = input.to_vec();
    let mut changed = false;

    for i in 0..=max_i {
        for j in 0..=max_j {
            match &input[i][j] {
                Space::Floor => {}
                seat @ (Space::Empty | Space::Occupied) => {
                    let occupied_seats_count = occupied_seats_count_fn(i, j, max_i, max_j);

                    match seat {
                        Space::Empty => {
                            if occupied_seats_count == 0 {
                                output[i][j] = Space::Occupied;
                                changed = true;
                            }
                        }
                        Space::Occupied => {
                            if occupied_seats_count >= occupied_seats_limit {
                                output[i][j] = Space::Empty;
                                changed = true;
                            }
                        }
                        Space::Floor => unreachable!(),
                    }
                }
            }
        }
    }

    (output, changed)
}

fn count_occupied_seats(input: &[Vec<Space>]) -> usize {
    input
        .iter()
        .map(|row| row.iter().filter(|&s| *s == Space::Occupied).count())
        .sum()
}

fn run_until_stable_state(input: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let mut map = input;

    loop {
        let (output, changed) = run(&map, 4, |i, j, max_i, max_j| {
            neighbour_offsets(i, j, max_i, max_j)
                .iter()
                .map(|&(dx, dy)| ((i as isize + dx) as usize, (j as isize + dy) as usize))
                .filter_map(|(x, y)| {
                    if map[x][y] == Space::Occupied {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .count()
        });
        map = output;

        if !changed {
            return map;
        }
    }
}

fn make_visible_seats_map(input: &[Vec<Space>]) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut map = HashMap::new();
    let max_i = input.len() - 1;
    let max_j = input[0].len() - 1;

    for i in 0..=max_i {
        for j in 0..=max_j {
            if input[i][j] == Space::Floor {
                continue;
            }

            for offset in neighbour_offsets(i, j, max_i, max_j) {
                let mut nseat = (i as isize + offset.0, j as isize + offset.1);

                while 0 <= nseat.0
                    && nseat.0 <= max_i as isize
                    && 0 <= nseat.1
                    && nseat.1 <= max_j as isize
                {
                    match input[nseat.0 as usize][nseat.1 as usize] {
                        Space::Floor => {
                            nseat.0 += offset.0;
                            nseat.1 += offset.1;
                        }
                        Space::Empty | Space::Occupied => {
                            map.entry((i, j))
                                .and_modify(|v: &mut Vec<(usize, usize)>| {
                                    v.push((nseat.0 as usize, nseat.1 as usize));
                                })
                                .or_insert(vec![(nseat.0 as usize, nseat.1 as usize)]);

                            break;
                        }
                    }
                }

                map.entry((i, j)).or_insert(vec![]);
            }
        }
    }

    map
}

fn run_until_stable_state2(input: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let visible_seats = make_visible_seats_map(&input);
    let mut map = input;

    loop {
        let (output, changed) = run(&map, 5, |i, j, _max_i, _max_j| {
            visible_seats[&(i, j)]
                .iter()
                .filter(|(ni, nj)| map[*ni][*nj] == Space::Occupied)
                .count()
        });
        map = output;

        if !changed {
            return map;
        }
    }
}

#[must_use]
pub fn solve_part_1(p: Problem) -> usize {
    let Problem { map } = p;
    let map = run_until_stable_state(map);

    count_occupied_seats(&map)
}

#[must_use]
pub fn solve_part_2(p: Problem) -> usize {
    let Problem { map } = p;
    let map = run_until_stable_state2(map);

    count_occupied_seats(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    const TEST_INPUT_VISIBLE_SEATS_1: &str = "\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

    const TEST_INPUT_VISIBLE_SEATS_2: &str = "\
.............
.L.L.#.#.#.#.
.............";

    const TEST_INPUT_VISIBLE_SEATS_3: &str = "\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_1(p), 37);
    }

    #[test]
    fn test_make_visible_seats_map_1() {
        let p: Problem = TEST_INPUT_VISIBLE_SEATS_1.parse().unwrap();
        let map = make_visible_seats_map(&p.map);

        assert_eq!(map[&(4, 3)].len(), 8);
    }

    #[test]
    fn test_make_visible_seats_map_2() {
        let p: Problem = TEST_INPUT_VISIBLE_SEATS_2.parse().unwrap();
        let map = make_visible_seats_map(&p.map);

        assert_eq!(map[&(1, 1)], vec![(1, 3)]);
    }

    #[test]
    fn test_make_visible_seats_map_3() {
        let p: Problem = TEST_INPUT_VISIBLE_SEATS_3.parse().unwrap();
        let map = make_visible_seats_map(&p.map);

        assert_eq!(map[&(3, 3)], vec![]);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(solve_part_2(p), 26);
    }
}
