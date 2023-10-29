use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
pub struct Problem {
    pub entries: Vec<i64>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries: Vec<_> = s.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Problem { entries })
    }
}

/// Assumes that `arr` is sorted
#[must_use]
pub fn find_two_sum(arr: &[i64], sum: i64) -> Option<(i64, i64)> {
    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        match (arr[i] + arr[j]).cmp(&sum) {
            Ordering::Less => i += 1,
            Ordering::Equal => return Some((arr[i], arr[j])),
            Ordering::Greater => j -= 1,
        }
    }

    None
}

#[must_use]
pub fn find_two_sum_naive(arr: &[i64], sum: i64) -> Option<(i64, i64)> {
    for (i, &a) in arr.iter().enumerate() {
        for &b in &arr[i..] {
            if b == sum - a {
                return Some((a, b));
            }
        }
    }

    None
}

/// Assumes that `arr` is sorted
#[must_use]
pub fn find_three_sum(arr: &[i64], sum: i64) -> Option<(i64, i64, i64)> {
    for (i, &a) in arr.iter().enumerate() {
        if let Some((b, c)) = find_two_sum(&arr[i..], sum - a) {
            return Some((a, b, c));
        }
    }

    None
}

#[must_use]
pub fn find_three_sum_naive(arr: &[i64], sum: i64) -> Option<(i64, i64, i64)> {
    for (i, &a) in arr.iter().enumerate() {
        if let Some((b, c)) = find_two_sum_naive(&arr[i..], sum - a) {
            return Some((a, b, c));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1721
979
366
299
675
1456";

    #[test]
    fn test_find_two_sum() {
        let Problem { mut entries } = TEST_INPUT.parse().unwrap();
        entries.sort();

        assert_eq!(
            find_two_sum(&entries, 2020).map(|(a, b)| a * b),
            Some(514579)
        );
    }

    #[test]
    fn test_find_three_sum() {
        let Problem { mut entries } = TEST_INPUT.parse().unwrap();
        entries.sort();

        assert_eq!(
            find_three_sum(&entries, 2020).map(|(a, b, c)| a * b * c),
            Some(241861950)
        );
    }

    #[test]
    fn test_find_two_sum_naive() {
        let Problem { entries } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            find_two_sum_naive(&entries, 2020).map(|(a, b)| a * b),
            Some(514579)
        );
    }

    #[test]
    fn test_find_three_sum_naive() {
        let Problem { entries } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            find_three_sum_naive(&entries, 2020).map(|(a, b, c)| a * b * c),
            Some(241861950)
        );
    }
}
