use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

#[derive(Debug)]
pub struct Problem {
    pub numbers: Vec<i64>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Problem { numbers })
    }
}

#[must_use]
pub fn find_first_not_following_the_rule(preamble_length: usize, numbers: &[i64]) -> (usize, i64) {
    let mut sum_counts: HashMap<i64, usize> = HashMap::new();

    let preamble = &numbers[..preamble_length];

    for i in 0..preamble_length {
        for j in (i + 1)..preamble_length {
            let sum = preamble[i] + preamble[j];
            sum_counts.entry(sum).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let mut idx = preamble_length;

    for window in numbers.windows(preamble_length + 1) {
        let new_arrival = window[preamble_length];

        if sum_counts.contains_key(&new_arrival) {
            idx += 1;
        } else {
            return (idx, new_arrival);
        }

        let staying_in_window = &window[1..preamble_length];
        let leaving_value = window[0];

        for v in staying_in_window {
            let leaving_sum = v + leaving_value;
            let joining_sum = v + new_arrival;

            match sum_counts.entry(leaving_sum) {
                Entry::Occupied(mut o) => {
                    let should_be_removed = *o.get() == 1;

                    if should_be_removed {
                        o.remove();
                    } else {
                        *o.get_mut() -= 1;
                    }
                }
                Entry::Vacant(_) => unreachable!(),
            }

            sum_counts
                .entry(joining_sum)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    unreachable!()
}

#[must_use]
pub fn find_continuous_set_summing_to_value(target: i64, numbers: &[i64]) -> Vec<i64> {
    let mut i = 0;
    let mut j = 1;
    let mut sum = numbers[i];

    while sum != target {
        if sum + numbers[j] <= target {
            sum += numbers[j];
            j += 1;
        } else {
            sum -= numbers[i];
            i += 1;
        }
    }

    numbers[i..j].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_find_first_not_following_the_rule() {
        let Problem { numbers } = TEST_INPUT.parse().unwrap();
        assert_eq!(find_first_not_following_the_rule(5, &numbers), (14, 127));
    }

    #[test]
    fn test_find_continuous_set_summing_to_value() {
        let Problem { numbers } = TEST_INPUT.parse().unwrap();
        assert_eq!(
            find_continuous_set_summing_to_value(127, &numbers),
            vec![15, 25, 47, 40]
        );
    }
}
