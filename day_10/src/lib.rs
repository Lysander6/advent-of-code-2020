use std::str::FromStr;

#[derive(Debug)]
pub struct Problem {
    numbers: Vec<u64>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Problem { numbers })
    }
}

fn count_differences(numbers: &[u64]) -> (u64, u64) {
    // assumes `numbers` is sorted

    let mut one_diff = 0;
    let mut three_diff = 0;

    // difference between charging outlet ("`numbers[0] - 0`")
    match numbers[0] {
        1 => one_diff += 1,
        3 => three_diff += 1,
        _ => {}
    }

    for window in numbers.windows(2) {
        let diff = window[1] - window[0];

        match diff {
            1 => one_diff += 1,
            3 => three_diff += 1,
            _ => {}
        }
    }

    (one_diff, three_diff + 1) // +1 for build in adapter
}

fn count_arrangements(numbers: &[u64]) -> usize {
    let diffs = numbers
        .iter()
        .scan(0, |prev, e| {
            let temp = *prev;
            *prev = *e;

            Some(*e - temp)
        })
        .collect::<Vec<_>>();

    let (x, y, z) = diffs
        .split(|n| *n == 3)
        .fold((0u32, 0u32, 0u32), |acc, s| match s.len() {
            // [3, 3] or [1] which can't be reduced
            0 | 1 => acc,

            // [a, a+1, a+2, a+5] - we keep a+1 or throw it out (2 choices)
            2 => (acc.0 + 1, acc.1, acc.2),

            // [a, a+1, a+2, a+3, a+6] - we keep all, we throw out a+1, we throw
            // out a+2, we throw out both a+1 and a+2 (4 choices)
            3 => (acc.0, acc.1 + 1, acc.2),

            // [a, a+1, a+2, a+3, a+4, a+7] - we keep all, we throw out a+1, we
            // throw out a+2, we throw out a+3, we throw out a+1 and a+2, we
            // throw out a+2 and a+3, we throw out a+1 and a+3 (7 choices)
            4 => (acc.0, acc.1, acc.2 + 1),

            // there are no other sequences in the input or test cases :^)
            _ => unreachable!(),
        });

    // choice of 2 doubles the number of correct arrangements, choice of 4
    // quadruples number of correct arrangements, and so on
    2usize.pow(x) * 4usize.pow(y) * 7usize.pow(z)
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> (u64, u64) {
    let Problem { numbers } = p;
    let mut numbers = numbers.clone();
    numbers.sort_unstable();

    count_differences(&numbers)
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> usize {
    let Problem { numbers } = p;
    let mut numbers = numbers.clone();
    numbers.sort_unstable();
    numbers.push(numbers[numbers.len() - 1] + 3);

    count_arrangements(&numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
16
10
15
5
1
11
7
19
6
12
4";

    const TEST_INPUT_LONG: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_solve_part_1() {
        let p1: Problem = TEST_INPUT.parse().unwrap();
        let p2: Problem = TEST_INPUT_LONG.parse().unwrap();

        assert_eq!(solve_part_1(&p1), (7, 5));
        assert_eq!(solve_part_1(&p2), (22, 10));
    }

    #[test]
    fn test_solve_part_2() {
        let p1: Problem = TEST_INPUT.parse().unwrap();
        let p2: Problem = TEST_INPUT_LONG.parse().unwrap();

        assert_eq!(solve_part_2(&p1), 8);
        assert_eq!(solve_part_2(&p2), 19208);
    }
}
