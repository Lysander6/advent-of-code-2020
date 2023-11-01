use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug, Default)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kv_pairs = s
            .lines()
            .flat_map(|l| l.split_ascii_whitespace())
            .collect::<Vec<_>>();

        let mut p = Passport::default();

        for kv in kv_pairs {
            match kv
                .split_once(':')
                .ok_or_else(|| anyhow!("malformed kv pair {}", kv))?
            {
                ("byr", v) => {
                    p.byr = Some(v.to_string());
                }
                ("iyr", v) => {
                    p.iyr = Some(v.to_string());
                }
                ("eyr", v) => {
                    p.eyr = Some(v.to_string());
                }
                ("hgt", v) => {
                    p.hgt = Some(v.to_string());
                }
                ("hcl", v) => {
                    p.hcl = Some(v.to_string());
                }
                ("ecl", v) => {
                    p.ecl = Some(v.to_string());
                }
                ("pid", v) => {
                    p.pid = Some(v.to_string());
                }
                ("cid", v) => {
                    p.cid = Some(v.to_string());
                }
                (k, _) => bail!("unknown key {}", k),
            }
        }

        Ok(p)
    }
}

#[derive(Debug)]
pub struct Problem {
    passports: Vec<Passport>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let passports = s
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Problem { passports })
    }
}

#[must_use]
pub fn passport_has_required_fields(p: &Passport) -> bool {
    let Passport {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid: _,
    } = p;

    byr.is_some()
        && iyr.is_some()
        && eyr.is_some()
        && hgt.is_some()
        && hcl.is_some()
        && ecl.is_some()
        && pid.is_some()
}

#[must_use]
pub fn count_passports_with_required_fields(p: &Problem) -> usize {
    let Problem { passports } = p;

    passports
        .iter()
        .filter(|p| passport_has_required_fields(p))
        .count()
}

#[must_use]
pub fn is_passport_valid(p: &Passport) -> bool {
    let Passport {
        byr,
        iyr,
        eyr,
        hgt,
        hcl,
        ecl,
        pid,
        cid: _,
    } = p;

    let Some(birth_year) = byr else {
        return false;
    };

    let birth_year = birth_year.parse::<u64>().unwrap_or(0);

    if (1920..=2002).contains(&birth_year) {
    } else {
        return false;
    }

    let Some(issue_year) = iyr else {
        return false;
    };

    let issue_year = issue_year.parse::<u64>().unwrap_or(0);

    if (2010..=2020).contains(&issue_year) {
    } else {
        return false;
    }

    let Some(expiry_year) = eyr else {
        return false;
    };

    let expiry_year = expiry_year.parse::<u64>().unwrap_or(0);

    if (2020..=2030).contains(&expiry_year) {
    } else {
        return false;
    }

    let Some(hgt) = hgt else {
        return false;
    };

    if hgt.ends_with("cm") {
        let Some(hgt) = hgt.strip_suffix("cm") else {
            return false;
        };

        let hgt = hgt.parse::<u64>().unwrap_or(0);

        if (150..=193).contains(&hgt) {
        } else {
            return false;
        }
    } else if hgt.ends_with("in") {
        let Some(hgt) = hgt.strip_suffix("in") else {
            return false;
        };

        let hgt = hgt.parse::<u64>().unwrap_or(0);

        if (59..=76).contains(&hgt) {
        } else {
            return false;
        }
    } else {
        return false;
    }

    let Some(hair_color) = hcl else {
        return false;
    };

    if hair_color.starts_with('#') {
        let Some(hair_color) = hair_color.strip_prefix('#') else {
            return false;
        };

        if hair_color.chars().all(|c| c.is_ascii_hexdigit()) {
        } else {
            return false;
        }
    } else {
        return false;
    }

    let Some(eye_color) = ecl else {
        return false;
    };

    match eye_color.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
        _ => {
            return false;
        }
    }

    let Some(pid) = pid else {
        return false;
    };

    if pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit()) {
    } else {
        return false;
    }

    true
}

#[must_use]
pub fn count_valid_passports(p: &Problem) -> usize {
    let Problem { passports } = p;

    passports
        .iter()
        .map(is_passport_valid)
        .filter(|valid| *valid)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const TEST_INVALID_PASSPORTS: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const TEST_VALID_PASSPORTS: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn test_count_passports_with_required_fields() {
        let p: Problem = TEST_INPUT.parse().unwrap();

        assert_eq!(count_passports_with_required_fields(&p), 2);
    }

    #[test]
    fn test_invalid_passports() {
        let Problem { passports } = TEST_INVALID_PASSPORTS.parse().unwrap();

        assert_eq!(passports.len(), 4);
        assert!(passports.iter().map(is_passport_valid).all(|valid| !valid));
    }

    #[test]
    fn test_valid_passports() {
        let Problem { passports } = TEST_VALID_PASSPORTS.parse().unwrap();

        assert_eq!(passports.len(), 4);
        assert!(passports.iter().map(is_passport_valid).all(|valid| valid));
    }

    #[test]
    fn test_count_valid_passports() {
        let p1: Problem = TEST_INVALID_PASSPORTS.parse().unwrap();
        let p2: Problem = TEST_VALID_PASSPORTS.parse().unwrap();

        assert_eq!(count_valid_passports(&p1), 0);
        assert_eq!(count_valid_passports(&p2), 4);
    }
}
