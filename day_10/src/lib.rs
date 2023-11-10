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

#[must_use]
pub fn solve_part_1(p: &Problem) -> (u64, u64) {
    let Problem { numbers } = p;
    let mut numbers = numbers.clone();
    numbers.sort_unstable();

    count_differences(&numbers)
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
}
