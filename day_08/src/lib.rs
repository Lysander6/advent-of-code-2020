use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Clone, Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr, value) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid instruction"))?;

        let value = value.trim_start_matches('+').parse::<i32>()?;

        Ok(match instr {
            "acc" => Instruction::Acc(value),
            "jmp" => Instruction::Jmp(value),
            "nop" => Instruction::Nop(value),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
pub struct Problem {
    instructions: Vec<Instruction>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Problem { instructions })
    }
}

/// # Errors
///
/// Returns error if instruction pointer `i` runs out of (positive) `i32` range
#[allow(clippy::cast_sign_loss)]
pub fn run_until_first_loop(p: &Problem) -> Result<(i32, bool), anyhow::Error> {
    let Problem { instructions } = p;
    let mut visited_lines = vec![false; instructions.len()];
    let mut acc = 0;
    let mut i = 0;

    loop {
        if i == instructions.len() || visited_lines[i] {
            break;
        }

        visited_lines[i] = true;

        let Some(instruction) = instructions.get(i) else {
            bail!("instruction pointer out of instructions range");
        };

        match instruction {
            Instruction::Acc(v) => {
                acc += v;
                i += 1;
            }
            Instruction::Jmp(v) => {
                i = i32::try_from(i)?.saturating_add(*v) as usize;
            }
            Instruction::Nop(_v) => {
                i += 1;
            }
        }
    }

    // program halts if `i` is set to index `insertions.len()`
    Ok((acc, i == instructions.len()))
}

/// # Errors
///
/// See `run_until_first_loop`
pub fn brute_force(p: &Problem) -> Result<i32, anyhow::Error> {
    // TODO: try backtracking instead
    for (i, instr) in p.instructions.iter().enumerate() {
        match instr {
            Instruction::Acc(_v) => continue,
            Instruction::Jmp(v) => {
                let mut instructions = p.instructions.clone();
                instructions[i] = Instruction::Nop(*v);
                let p = Problem { instructions };
                let (acc, halts) = run_until_first_loop(&p)?;
                if halts {
                    return Ok(acc);
                }
            }
            Instruction::Nop(v) => {
                let mut instructions = p.instructions.clone();
                instructions[i] = Instruction::Jmp(*v);
                let p = Problem { instructions };
                let (acc, halts) = run_until_first_loop(&p)?;
                if halts {
                    return Ok(acc);
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    const TEST_INPUT_HALTING: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";

    #[test]
    fn test_run_until_first_loop() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(run_until_first_loop(&p).unwrap(), (5, false));
    }

    #[test]
    fn test_run_until_first_loop_halting() {
        let p: Problem = TEST_INPUT_HALTING.parse().unwrap();
        assert_eq!(run_until_first_loop(&p).unwrap(), (8, true));
    }

    #[test]
    fn test_brute_force() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(brute_force(&p).unwrap(), 8);
    }
}
