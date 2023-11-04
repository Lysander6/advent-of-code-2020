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

fn count_unique(group: &str) -> usize {
    let group_unique_answers = group.chars().filter(|c| *c != '\n').collect::<HashSet<_>>();

    group_unique_answers.len()
}

fn count_shared(group: &str) -> usize {
    let answers = group
        .lines()
        .map(|answer| answer.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let shared_answers = answers
        .into_iter()
        .reduce(|acc, set| acc.intersection(&set).copied().collect::<HashSet<_>>())
        .map(|set| set.len());

    shared_answers.unwrap_or(0)
}

#[must_use]
pub fn sum_unique_answers(p: &Problem) -> usize {
    let Problem { groups } = p;

    groups.iter().map(|g| count_unique(g)).sum()
}

#[must_use]
pub fn sum_shared_answers(p: &Problem) -> usize {
    let Problem { groups } = p;

    groups.iter().map(|g| count_shared(g)).sum()
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
    fn test_sum_unique_answers() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(sum_unique_answers(&p), 11);
    }

    #[test]
    fn test_sum_shared_answers() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(sum_shared_answers(&p), 6);
    }
}
