use std::collections::{HashMap, HashSet};

const RAW: &str = include_str!("../input.txt");
type Coord = (i32, i32);
type Elevation = usize;
struct Input {
    map: HashMap<Coord, Elevation>,
}

impl Input {
    fn next_steps(&self, coord: Coord) -> Vec<Coord> {
        let mut neighbors = vec![];
        let current_elevation = self.map[&coord];
        for dir in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            let next_coord = next_coord_in_dir(coord, &dir);
            if let Some(next_elevation) = self.map.get(&next_coord) {
                if *next_elevation == current_elevation + 1 {
                    neighbors.push(next_coord);
                }
            }
        }
        neighbors
    }

    fn trailheads(&self) -> Vec<Coord> {
        self.map
            .iter()
            .filter_map(
                |(coord, elevation)| {
                    if *elevation == 0 {
                        Some(*coord)
                    } else {
                        None
                    }
                },
            )
            .collect::<Vec<_>>()
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_input(raw: &str) -> Input {
    let mut rows = 0;
    let mut cols = 0;
    let mut map = HashMap::new();
    for (row, line) in raw.lines().enumerate() {
        if row >= rows {
            rows = row + 1
        }
        for (col, num) in line.chars().enumerate() {
            if col >= cols {
                cols = col + 1
            }
            map.insert(
                (row as i32, col as i32),
                num.to_digit(10).expect("could not convert to digit") as usize,
            );
        }
    }
    Input { map }
}

fn next_coord_in_dir(coord: Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Up => (coord.0 - 1, coord.1),
        Direction::Right => (coord.0, coord.1 + 1),
        Direction::Down => (coord.0 + 1, coord.1),
        Direction::Left => (coord.0, coord.1 - 1),
    }
}

fn walk_path(input: &Input, coord: Coord, trail_ends: &mut HashSet<Coord>, rating: &mut usize) {
    //  end of path
    if input.map[&coord] == 9 {
        trail_ends.insert(coord); // add coord to list of trail ends
        *rating += 1; // increment rating for path
        return; // stop recursion
    }

    // For each valid neighbour, recurse
    for c in input.next_steps(coord) {
        walk_path(input, c, trail_ends, rating)
    }
}

fn part1(input: &Input) -> usize {
    let mut score = 0_usize;
    for trailhead in input.trailheads() {
        let mut trail_ends = HashSet::new();
        walk_path(input, trailhead, &mut trail_ends, &mut 0);
        score += trail_ends.len()
    }
    score
}

fn part2(input: &Input) -> usize {
    let mut rating = 0_usize;
    for trailhead in input.trailheads() {
        let mut trail_ends = HashSet::new();
        walk_path(input, trailhead, &mut trail_ends, &mut rating);
    }
    rating
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_input() {
        let input = parse_input(SAMPLE);
        assert_eq!(input.map.len(), 8 * 8);
        assert_eq!(input.map[&(1, 1)], 8);
        assert_eq!(input.map[&(1, 2)], 1);
        assert_eq!(input.map[&(2, 1)], 7);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = 36;

        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SAMPLE);
        let expected = 81;

        assert_eq!(part2(&input), expected);
    }
}
