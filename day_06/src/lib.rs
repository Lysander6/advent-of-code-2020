use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
pub struct Problem {
    groups: Vec<String>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = s.split("\n\n").map(str::to_string).collect::<Vec<_>>();

        Ok(Problem { groups })
    }
}

fn count_unique(s: &str) -> usize {
    let mut set = s.chars().collect::<HashSet<_>>();

    set.remove(&'\n');

    set.len()
}

#[must_use]
pub fn sum_yes_answers(p: &Problem) -> usize {
    let Problem { groups } = p;

    groups.iter().map(|s| count_unique(s)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_sum_yes_answers() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(sum_yes_answers(&p), 11);
    }
}
