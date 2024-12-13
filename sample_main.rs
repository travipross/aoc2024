const RAW: &str = include_str!("../input.txt");
type Input = ();

fn parse_input(raw: &str) -> Input {
    todo!()
}

fn part1(input: &Input) -> usize {
    todo!()
}

fn part2(input: &Input) -> usize {
    todo!()
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod dayXYZ_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_input() {
        let input = parse_input(SAMPLE);
        todo!()
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = todo!();

        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SAMPLE);
        let expected = todo!();

        assert_eq!(part2(&input), expected);
    }
}
