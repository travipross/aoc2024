const RAW: &str = include_str!("../input.txt");

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

impl Instruction {
    fn exec(&self) -> u32 {
        match self {
            Self::Mul(a, b) => a * b,
            _ => 0,
        }
    }
}

fn part1(instructions: &Vec<Instruction>) -> u32 {
    instructions.iter().map(|i| i.exec()).sum()
}

fn part2(instructions: &Vec<Instruction>) -> u32 {
    let mut enabled_instructions = vec![];
    let mut enabled = true;
    for i in instructions.iter() {
        match i {
            Instruction::Mul(_, _) => {
                if enabled {
                    enabled_instructions.push(i.clone());
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        };
    }
    part1(&enabled_instructions)
}

fn parse_raw_instructions(raw_instructions: &str) -> Vec<Instruction> {
    // https://regex101.com/r/PGwdrv/2
    let pattern = regex::RegexBuilder::new(r"(mul|do|don\'t)\((?:(\d{1,3}),(\d{1,3}))?\)")
        .build()
        .expect("could not build regex");
    pattern
        .captures_iter(raw_instructions)
        .map(|c| {
            let op = &c[1];
            match op {
                "mul" => {
                    let n1 = &c[2];
                    let n2 = &c[3];
                    Instruction::Mul(
                        n1.parse().expect("could not parse operand"),
                        n2.parse().expect("could not parse operand"),
                    )
                }
                "do" => Instruction::Do,
                "don't" => Instruction::Dont,
                _ => panic!("unknown op: {op}"),
            }
        })
        .collect()
}

fn main() {
    let instructions = parse_raw_instructions(RAW);
    println!(
        "Part 1: The result of the instructions is: {}",
        part1(&instructions)
    );

    println!(
        "Part 2: The result of the enabled instructions is: {}",
        part2(&instructions)
    );
}

#[cfg(test)]
mod day3_tests {
    use crate::{parse_raw_instructions, part1, part2, Instruction};

    const SAMPLE1: &str = include_str!("../sample1.txt");
    const SAMPLE2: &str = include_str!("../sample2.txt");

    #[test]
    fn test_parse_sample_1() {
        let parsed = parse_raw_instructions(SAMPLE1);
        let expected = vec![
            Instruction::Mul(2, 4),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Mul(8, 5),
        ];

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_sample_2() {
        let parsed = parse_raw_instructions(SAMPLE2);
        let expected = vec![
            Instruction::Mul(2, 4),
            Instruction::Dont,
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Do,
            Instruction::Mul(8, 5),
        ];

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_part1() {
        let instructions = parse_raw_instructions(SAMPLE1);
        assert_eq!(part1(&instructions), 161)
    }

    #[test]
    fn test_part2() {
        let instructions = parse_raw_instructions(SAMPLE2);
        assert_eq!(part2(&instructions), 48)
    }
}
