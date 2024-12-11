use std::iter::zip;

/// Read raw string input as two columns of unisigned integers
fn parse_str(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let n1 = parts
                .next()
                .expect("couldn't get first digit")
                .parse::<u32>()
                .expect("couldn't parse as digit");
            let n2 = parts
                .next()
                .expect("couldn't get first digit")
                .parse::<u32>()
                .expect("couldn't parse as digit");
            (n1, n2)
        })
        .collect::<(Vec<_>, Vec<_>)>()
}

/// Calculate total distance
fn part1(input: &str) -> u32 {
    let (mut left_col, mut right_col) = parse_str(input);

    // Sort each column
    left_col.sort();
    right_col.sort();

    // Calculate sum of absolute difference between each corresponding item
    zip(&left_col, &right_col)
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

/// Calculate "similarity score"
fn part2(input: &str) -> u32 {
    let (left_col, right_col) = parse_str(input);

    // For each item in left list, multiply its value by the number of times that item
    // appears in right list
    left_col
        .into_iter()
        .map(|l| right_col.clone().into_iter().filter(|r| *r == l).count() as u32 * l)
        .sum()
}

fn main() {
    // Read and parse input
    let input1_txt = include_str!("../input.txt");

    // // First answer
    let ans1 = part1(&input1_txt);
    println!("Part 1: {ans1}");

    // 2nd answer
    let ans2 = part2(input1_txt);
    println!("Part 2: {ans2}");
}

#[cfg(test)]
mod day1_tests {
    use crate::{part1, part2};

    const SAMPLE_INPUT: &str = include_str!("../sample.txt");

    #[test]
    fn part1_test() {
        assert_eq!(part1(SAMPLE_INPUT), 11);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(SAMPLE_INPUT), 31);
    }
}
