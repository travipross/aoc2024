use cached::proc_macro::cached;

const RAW: &str = include_str!("../input.txt");
type Input = Vec<usize>;

fn parse_input(raw: &str) -> Input {
    raw.split_whitespace()
        .map(|c| c.parse().expect("could not parse as number"))
        .collect()
}

// return true if number of digits is even
fn has_even_digits(n: usize) -> bool {
    n_digits(n) % 2 == 0
}

// count digits in stone value
fn n_digits(n: usize) -> u32 {
    n.ilog10() + 1
}

// split number into two numbers
fn split_digits(n: usize) -> Vec<usize> {
    let divisor = 10_usize.pow(n_digits(n) / 2);
    vec![n / divisor, n % divisor]
}

// blink at a given stone a given number of times, applying rules as appropriate, counting total number of resulting stones
// recurse for given number of blinks on each resulting stone
// lots of repeating patterns expected, so we cache the result of blinking a given stone n-times
#[cached]
fn recursive_count_stone_after_blinks(stone: usize, n_blinks: usize) -> usize {
    if n_blinks == 0 {
        1
    } else {
        blink_at_stone(stone)
            .iter()
            .map(|s| recursive_count_stone_after_blinks(*s, n_blinks - 1))
            .sum()
    }
}

// get the total number of stones after given number of blinks
fn count_all_stones_after_blinks(stones: &Input, n_blinks: usize) -> usize {
    stones.into_iter().fold(0, |acc, &elem| {
        acc + recursive_count_stone_after_blinks(elem, n_blinks)
    })
}

// Apply blink rules to a single stone
fn blink_at_stone(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if has_even_digits(stone) {
        split_digits(stone)
    } else {
        vec![stone * 2024]
    }
}

fn part1(input: &Input) -> usize {
    count_all_stones_after_blinks(input, 25)
}

fn part2(input: &Input) -> usize {
    count_all_stones_after_blinks(input, 75)
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_input() {
        let input = parse_input("1 2024 1 0 9 9 2021976");
        assert_eq!(input, vec![1, 2024, 1, 0, 9, 9, 2021976])
    }

    #[test_case::test_case(125, 1 => 1)]
    #[test_case::test_case(125, 2 => 2)]
    #[test_case::test_case(125, 3 => 2)]
    #[test_case::test_case(125, 4 => 3)]
    #[test_case::test_case(125, 5 => 5)]
    fn test_recursive_blink_stone(stone: usize, n_blinks: usize) -> usize {
        recursive_count_stone_after_blinks(stone, n_blinks)
    }

    #[test_case::test_case(1 => false)]
    #[test_case::test_case(12 => true)]
    #[test_case::test_case(123 => false)]
    #[test_case::test_case(1234 => true)]
    #[test_case::test_case(12345 => false)]
    #[test_case::test_case(123456 => true)]
    fn test_even_digits(n: usize) -> bool {
        has_even_digits(n)
    }

    #[test_case::test_case(1234 => vec![12,34]) ]
    #[test_case::test_case(123456 => vec![123,456]) ]
    #[test_case::test_case(111000 => vec![111,0]) ]
    #[test_case::test_case(111002 => vec![111,2]) ]
    fn test_split_digits(n: usize) -> Vec<usize> {
        split_digits(n)
    }

    #[test_case::test_case(125 => vec![253000])]
    #[test_case::test_case(253000 => vec![253, 0])]
    #[test_case::test_case(1 => vec![2024])]
    #[test_case::test_case(7 => vec![14168])]
    fn test_blink_at_stone(stone: usize) -> Vec<usize> {
        blink_at_stone(stone)
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = 55312;

        assert_eq!(part1(&input), expected);
    }
}
