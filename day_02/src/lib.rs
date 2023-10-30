use std::str::FromStr;

use parse_display::{Display, FromStr};

#[derive(Debug, Display, FromStr)]
#[display("{min}-{max} {letter}: {password}")]
pub struct PasswordWithPolicy {
    password: String,
    letter: char,
    min: usize,
    max: usize,
}

pub struct Problem {
    pub passwords: Vec<PasswordWithPolicy>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem {
            passwords: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

#[must_use]
pub fn validate_password(p: &PasswordWithPolicy) -> bool {
    let PasswordWithPolicy {
        password, letter, ..
    } = p;
    let PasswordWithPolicy { min, max, .. } = *p;
    let count = password.chars().filter(|c| c == letter).count();

    min <= count && count <= max
}

#[must_use]
pub fn validate_password_new_policy(p: &PasswordWithPolicy) -> bool {
    let PasswordWithPolicy { password, .. } = p;
    let PasswordWithPolicy {
        letter, min, max, ..
    } = *p;

    let Some(m) = password.chars().nth(min - 1) else {
        return false;
    };
    let Some(n) = password.chars().nth(max - 1) else {
        return false;
    };

    (m == letter || n == letter) && m != n
}

#[must_use]
pub fn count_valid_passwords<F>(passwords: &[PasswordWithPolicy], validation_fn: F) -> usize
where
    F: Fn(&PasswordWithPolicy) -> bool,
{
    passwords.iter().filter(|p| validation_fn(p)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_count_valid_passwords_with_old_policy() {
        let Problem { passwords } = TEST_INPUT.parse().unwrap();

        assert_eq!(count_valid_passwords(&passwords, validate_password), 2);
    }

    #[test]
    fn test_count_valid_passwords_with_new_policy() {
        let Problem { passwords } = TEST_INPUT.parse().unwrap();

        assert_eq!(
            count_valid_passwords(&passwords, validate_password_new_policy),
            1
        );
    }
}
