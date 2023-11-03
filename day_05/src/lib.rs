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
    pub boarding_passes: Vec<String>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem {
            boarding_passes: s.lines().map(str::to_string).collect(),
        })
    }
}

/// # Errors
///
/// Returns error when input slice length is greater than `u16::MAX`
pub fn find_my_seat_id(occupied_seats: &[u16]) -> Result<u16, anyhow::Error> {
    let mut occupied_seats = occupied_seats.to_vec();
    occupied_seats.sort_unstable();

    let idx_offset = occupied_seats[0];

    for (i, &seat_id) in occupied_seats.iter().enumerate() {
        if seat_id - u16::try_from(i)? != idx_offset {
            return Ok(seat_id - 1);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_find_my_seat_id() {
        assert_eq!(find_my_seat_id(&[9, 4, 8, 5, 6]).unwrap(), 7);
    }
}
