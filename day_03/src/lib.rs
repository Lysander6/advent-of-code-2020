use std::str::FromStr;

#[derive(Debug)]
pub struct Problem {
    map: Vec<bool>,
    width: usize,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem {
            map: s
                .lines()
                .flat_map(|l| l.chars().map(|c| c == '#'))
                .collect(),
            width: s.lines().nth(0).unwrap().len(),
        })
    }
}

#[must_use]
pub fn count_collisions(p: &Problem, right: usize, down: usize) -> usize {
    let Problem { map, width } = p;
    let mut dx = 0;
    let mut dy = 0;
    let mut i = 0;
    let mut hits = 0;

    while i < map.len() {
        if map[i] {
            hits += 1;
        }

        dx = (dx + right) % width;
        dy += width * down;
        i = dx + dy;
    }

    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_count_collisions() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(count_collisions(&p, 1, 1), 2);
        assert_eq!(count_collisions(&p, 3, 1), 7);
        assert_eq!(count_collisions(&p, 5, 1), 3);
        assert_eq!(count_collisions(&p, 7, 1), 4);
        assert_eq!(count_collisions(&p, 1, 2), 2);
    }
}
