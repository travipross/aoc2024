type Level = u8;
type Report = Vec<Level>;

fn levels_trending(report: &Report) -> bool {
    report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| b < a)
}

fn levels_diff_ok(report: &Report) -> bool {
    report.is_sorted_by(|a, b| {
        let diff = a.abs_diff(*b);
        1 <= diff && diff <= 3
    })
}

fn dampener<F>(f: F, report: &Report) -> bool
where
    F: Fn(&Report) -> bool,
{
    for i in 0..report.len() {
        let mut tmp = report.clone();
        tmp.remove(i);
        if f(&tmp) {
            return true;
        }
    }
    return false;
}

fn parse_input(raw: &str) -> Vec<Report> {
    raw.lines()
        .map(|line| -> Report {
            let report = line
                .split_whitespace()
                .map(|num| -> Level { num.parse().expect("couldn't parse level") })
                .collect::<Report>();
            report
        })
        .collect::<Vec<Report>>()
}

fn part1(reports: &Vec<Report>) -> usize {
    let safe_reports = reports
        .into_iter()
        .filter(|&r| levels_trending(r) && levels_diff_ok(r))
        .collect::<Vec<_>>();
    safe_reports.len()
}

fn part2(reports: &Vec<Report>) -> usize {
    let safe_reports = reports
        .into_iter()
        .filter(|&r| dampener(|r| levels_trending(&r) && levels_diff_ok(&r), r))
        .collect::<Vec<_>>();
    safe_reports.len()
}

fn main() {
    let raw = include_str!("../input.txt");
    let reports = parse_input(raw);

    println!("Part 1: {} reports are safe", part1(&reports));
    println!("Part 2: {} reports are safe", part2(&reports));
}

#[cfg(test)]
mod day2_tests {
    use crate::{parse_input, part1, part2};

    const RAW: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse() {
        let parsed = parse_input(RAW);
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_part1() {
        let reports = parse_input(RAW);

        assert_eq!(part1(&reports), 2);
    }

    #[test]
    fn test_part2() {
        let reports = parse_input(RAW);

        assert_eq!(part2(&reports), 4);
    }
}
