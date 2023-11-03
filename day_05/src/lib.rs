use std::str::FromStr;

#[must_use]
pub fn str_to_seat(s: &str) -> u16 {
    let mut a = 0;

    for (i, c) in s.chars().rev().enumerate() {
        match c {
            'B' | 'R' => {
                a |= 1 << i;
            }
            'F' | 'L' => {}
            _ => unreachable!(),
        }
    }

    a
}

#[derive(Debug)]
pub struct Problem {
    boarding_passes: Vec<String>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem {
            boarding_passes: s.lines().map(str::to_string).collect(),
        })
    }
}

#[must_use]
pub fn find_max_seat_id(p: &Problem) -> Option<u16> {
    let Problem { boarding_passes } = p;

    boarding_passes.iter().map(|s| str_to_seat(s)).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test_bin_encoding() {
        // BFFFBBFRRR
        let row = 0b1000110u16;
        let col = 0b111u16;

        assert_eq!(row, 70);
        assert_eq!(col, 7);
        assert_eq!((row << 3) | col, 567);
        assert_eq!(0b1000110111, 567);
    }

    #[test]
    fn test_str_to_seat() {
        assert_eq!(str_to_seat("BFFFBBFRRR"), 567);
        assert_eq!(str_to_seat("FFFBBBFRRR"), 119);
        assert_eq!(str_to_seat("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_find_max_seat_id() {
        let p = TEST_INPUT.parse().unwrap();

        assert_eq!(find_max_seat_id(&p), Some(820));
    }
}
