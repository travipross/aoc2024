const RAW: &str = include_str!("../input.txt");

type Rule = (u32, u32);
type Update = Vec<Page>;
type Page = u32;

fn parse_input(raw: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut rules = vec![];
    let mut updates = vec![];

    for l in raw.lines() {
        if l.contains("|") {
            let (a, b) = l.split_once("|").expect("could not split rule");
            rules.push((
                a.parse().expect("could not parse num"),
                b.parse().expect("could not parse num"),
            ))
        } else if l.contains(",") {
            updates.push(
                l.split(",")
                    .map(|c| c.parse::<u32>().expect("could not parse num"))
                    .collect(),
            )
        }
    }

    (rules, updates)
}

fn is_right_order(update: &Update, rules: &Vec<Rule>) -> bool {
    rules.iter().all(|r| update_satisfies_rule(update, r))
}

fn update_satisfies_rule(update: &Update, rule: &Rule) -> bool {
    let violated_rule = if let (Some(lower), Some(higher)) = (
        update.iter().position(|&e| e == rule.0),
        update.iter().position(|&e| e == rule.1),
    ) {
        lower > higher
    } else {
        false
    };

    // println!("{update:?}, {rule:?}, violated: {violated_rule}");
    !violated_rule
}

fn middle_page_from_update(update: &Update) -> Page {
    let l = update.len();
    update[l / 2]
}

fn reorder_pages_from_update(update: &Update, rules: &Vec<Rule>) -> Update {
    todo!()
}

fn part1(rules: &Vec<Rule>, updates: &Vec<Update>) -> u32 {
    updates
        .iter()
        .filter(|&u| is_right_order(u, rules))
        .map(|u| middle_page_from_update(u))
        .sum()
}

fn part2(rules: &Vec<Rule>, updates: &Vec<Update>) -> u32 {
    updates
        .iter()
        .filter(|&u| !is_right_order(u, rules))
        .map(|u| middle_page_from_update(&reorder_pages_from_update(u, rules)))
        .sum()
}

fn main() {
    let (rules, updates) = parse_input(RAW);
    println!("Part 1: {}", part1(&rules, &updates));
}

#[cfg(test)]
mod day5_tests {
    use super::*;
    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_sample() {
        let (rules, updates) = parse_input(SAMPLE);

        assert_eq!(rules.len(), 21);
        assert_eq!(rules[3], (97, 47));
        assert_eq!(updates.len(), 6);
        assert_eq!(updates[3], vec![75, 97, 47, 61, 53])
    }

    #[test]
    fn test_middle_page_from_update() {
        let u: Update = vec![4, 6, 2, 8, 7];

        assert_eq!(middle_page_from_update(&u), 2);
    }

    #[test]
    fn test_satisfied_rule() {
        let rule = (97, 75);
        let update = vec![75, 97, 47, 61, 53];

        assert!(!update_satisfies_rule(&update, &rule))
    }

    #[test]
    fn test_reorder_pages_from_update() {
        let (rules, _) = parse_input(SAMPLE);
        let update = vec![75, 97, 47, 61, 53];

        assert_eq!(
            reorder_pages_from_update(&update, &rules),
            vec![97, 75, 47, 61, 53]
        )
    }

    #[test]
    fn test_part1() {
        let (rules, updates) = parse_input(SAMPLE);

        assert_eq!(part1(&rules, &updates), 143)
    }

    #[test]
    fn test_part2() {
        let (rules, updates) = parse_input(SAMPLE);

        assert_eq!(part2(&rules, &updates), 123)
    }
}
