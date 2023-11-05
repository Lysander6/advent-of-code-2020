use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Context};

#[derive(Debug)]
pub struct Problem<'a> {
    name_to_idx: HashMap<&'a str, usize>,
    #[allow(dead_code)]
    idx_to_name: Vec<&'a str>,
    contains: Vec<Vec<(usize, usize)>>,
    contained_by: Vec<Vec<(usize, usize)>>,
}

impl<'a> TryFrom<&'a str> for Problem<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut next_bag_idx = 0usize;
        let mut name_to_idx: HashMap<&str, usize> = HashMap::new();
        let mut idx_to_name: Vec<&str> = Vec::new();
        let mut contains: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut contained_by: Vec<Vec<(usize, usize)>> = Vec::new();

        for line in s.lines() {
            let (containing_bag_name, contents) = line
                .split_once(" bags contain ")
                .ok_or_else(|| anyhow!("malformed input"))?;

            let contained_bags = if contents == "no other bags." {
                Vec::new()
            } else {
                contents
                    .split_terminator(", ")
                    .map(|z| {
                        let first_space_idx = z.find(' ').unwrap();
                        let count = z[0..first_space_idx].parse::<usize>().unwrap();

                        let idx_past_bag_name = z.find(" bag").unwrap();
                        let bag_name = &z[(first_space_idx + 1)..idx_past_bag_name];

                        (count, bag_name)
                    })
                    .collect()
            };

            let containing_bag_idx = name_to_idx
                .entry(containing_bag_name)
                .or_insert_with(|| {
                    let idx = next_bag_idx;
                    next_bag_idx += 1;
                    idx
                })
                .to_owned();

            if containing_bag_idx == idx_to_name.len() {
                // new bag
                idx_to_name.push(containing_bag_name);
                contains.push(Vec::new());
                contained_by.push(Vec::new());
            }

            for (contained_times, contained_bag_name) in contained_bags {
                let contained_bag_idx = name_to_idx
                    .entry(contained_bag_name)
                    .or_insert_with(|| {
                        let idx = next_bag_idx;
                        next_bag_idx += 1;
                        idx
                    })
                    .to_owned();

                if contained_bag_idx == idx_to_name.len() {
                    // new bag
                    // TODO: move to lambda function
                    idx_to_name.push(contained_bag_name);
                    contains.push(Vec::new());
                    contained_by.push(Vec::new());
                }

                contains[containing_bag_idx].push((contained_bag_idx, contained_times));
                contained_by[contained_bag_idx].push((containing_bag_idx, contained_times));
            }
        }

        Ok(Problem {
            name_to_idx,
            idx_to_name,
            contains,
            contained_by,
        })
    }
}

/// # Errors
///
/// Returns error if bag with `bag_name` does not exist
pub fn count_containing_bags(p: &Problem, bag_name: &str) -> Result<usize, anyhow::Error> {
    let Problem {
        name_to_idx,
        contained_by,
        ..
    } = p;

    let bag_idx = name_to_idx.get(bag_name).context("bag name not found")?;

    let mut containers = contained_by[*bag_idx]
        .iter()
        .map(|&(bag_idx, _)| bag_idx)
        .collect::<HashSet<usize>>();

    let mut containers_to_explore = containers.clone();

    while !containers_to_explore.is_empty() {
        let mut z = HashSet::new();
        for c in &containers_to_explore {
            let a = contained_by[*c]
                .iter()
                .map(|&(bag_idx, _)| bag_idx)
                .collect::<HashSet<_>>();
            containers = containers.union(&a).copied().collect();
            z = z.union(&a).copied().collect();
        }
        containers_to_explore = z;
    }

    Ok(containers.len())
}

fn count_contained(contains: &Vec<Vec<(usize, usize)>>, bag_idx: usize) -> usize {
    let contained_bags = &contains[bag_idx];

    if contained_bags.is_empty() {
        return 0;
    }

    contained_bags
        .iter()
        .map(|&(bag_idx, count)| count + count * count_contained(contains, bag_idx))
        .sum()
}

/// # Errors
///
/// Returns error if bag with `bag_name` does not exist
pub fn count_contained_bags(p: &Problem, bag_name: &str) -> Result<usize, anyhow::Error> {
    let Problem {
        name_to_idx,
        contains,
        ..
    } = p;

    let bag_idx = name_to_idx.get(bag_name).context("bag name not found")?;

    Ok(count_contained(contains, *bag_idx))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const TEST_INPUT_NESTED: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_problem_parser() {
        assert!(Problem::try_from(TEST_INPUT).is_ok());
    }

    #[test]
    fn test_count_containing_bags() {
        let p: Problem = TEST_INPUT.try_into().unwrap();
        assert_eq!(count_containing_bags(&p, "shiny gold").unwrap(), 4);
    }

    #[test]
    fn test_count_contained_bags() {
        let p: Problem = TEST_INPUT.try_into().unwrap();
        assert_eq!(count_contained_bags(&p, "shiny gold").unwrap(), 32);
    }

    #[test]
    fn test_count_contained_bags_deeply_nested() {
        let p: Problem = TEST_INPUT_NESTED.try_into().unwrap();
        assert_eq!(count_contained_bags(&p, "shiny gold").unwrap(), 126);
    }
}
