use std::iter::zip;

use itertools::{repeat_n, Itertools};

const RAW: &str = include_str!("../input.txt");
type Target = usize;
type Operand = usize;
type Input = Vec<(Target, Vec<Operand>)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn get_combo_iter(item_vals: Vec<Operator>, n_items: usize) -> impl Iterator<Item = Vec<Operator>> {
    repeat_n(item_vals, n_items).multi_cartesian_product() // permutations with replacement
}

fn parse_input(raw: &str) -> Input {
    let mut input = Input::new();

    for line in raw.lines() {
        let mut iter = line.split(':');
        let target = iter
            .next()
            .expect("could not get target")
            .parse::<Target>()
            .expect("could not parse value");
        let operands = iter
            .next()
            .expect("could not get target")
            .split_whitespace()
            .map(|s| s.parse::<Operand>().expect("could not parse value"))
            .collect::<Vec<_>>();
        input.push((target, operands));
    }

    input
}

fn evaluate(operands: &Vec<Operand>, operators: Vec<Operator>) -> usize {
    let initial_val = operands[0];
    zip(operands[1..].iter(), operators).fold(
        initial_val,
        |acc, (operand, operator)| match operator {
            Operator::Add => acc + operand,
            Operator::Multiply => acc * operand,
            Operator::Concatenate => format!("{acc}{operand}").parse().expect("could not parse"),
        },
    )
}

fn equation_is_solvable(target: Target, operands: &Vec<Operand>, operators: Vec<Operator>) -> bool {
    for operators in get_combo_iter(operators, operands.len() - 1) {
        if evaluate(operands, operators) == target {
            return true;
        }
    }
    false
}

fn calibration_result(input: &Input, operators: Vec<Operator>) -> usize {
    input
        .into_iter()
        .filter_map(|(target, operands)| {
            if equation_is_solvable(*target, &operands, operators.clone()) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn part1(input: &Input) -> usize {
    calibration_result(input, vec![Operator::Add, Operator::Multiply])
}

fn part2(input: &Input) -> usize {
    calibration_result(
        input,
        vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
    )
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test_case::test_case(vec![10, 19], vec![Operator::Add] => 29)]
    #[test_case::test_case(vec![10, 19], vec![Operator::Multiply] => 190)]
    #[test_case::test_case(vec![81,40,27], vec![Operator::Multiply, Operator::Add] => 3267)]
    #[test_case::test_case(vec![81,40,27], vec![Operator::Add, Operator::Multiply] => 3267)]
    #[test_case::test_case(vec![11, 6, 16, 20], vec![Operator::Add, Operator::Multiply, Operator::Add] => 292)]
    #[test_case::test_case(vec![11, 6, 16, 20], vec![Operator::Concatenate, Operator::Concatenate, Operator::Concatenate] => 1161620)]
    fn test_evaluate(operands: Vec<Operand>, operators: Vec<Operator>) -> usize {
        evaluate(&operands, operators)
    }

    #[test_case::test_case(190, vec![10, 19] => true)]
    #[test_case::test_case(3267, vec![81,40,27] => true)]
    #[test_case::test_case(292, vec![11,6,16,20] => true)]
    #[test_case::test_case(161011, vec![16,10,13] => false)]
    fn test_equation_is_solvable(target: Target, operands: Vec<Operand>) -> bool {
        equation_is_solvable(target, &operands, vec![Operator::Add, Operator::Multiply])
    }

    #[test_case::test_case(1)]
    #[test_case::test_case(2)]
    #[test_case::test_case(3)]
    #[test_case::test_case(4)]
    fn test_operator_combos(n_operands: usize) {
        // try parallel with rayon
        let combos = get_combo_iter(
            vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
            n_operands,
        )
        .collect::<Vec<Vec<_>>>();
        assert_eq!(combos.len(), 3_usize.pow(n_operands as u32));
        assert_eq!(combos.iter().unique().cloned().collect::<Vec<_>>(), combos)
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(SAMPLE);
        assert_eq!(input.len(), 9);
        assert_eq!(input[5], (161011, vec![16, 10, 13]))
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = 3749;

        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SAMPLE);
        let expected = 11387;

        assert_eq!(part2(&input), expected);
    }
}
