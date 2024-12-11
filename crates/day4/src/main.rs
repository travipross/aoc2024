mod coord;
mod direction;
use std::{collections::HashMap, ops::Not};

use coord::Coord;
use direction::{Direction, DirectionHoriz, DirectionVert};
const RAW: &str = include_str!("../input.txt");

fn main() {
    let puzzle = parse_input(RAW);
    println!("Part 1: XMAS found {} times", part1(puzzle.clone()));
    println!("Part 2: X-MAS found {} times", part2(&puzzle));
}

#[derive(Clone, PartialEq, Debug, Eq)]
struct Puzzle {
    vals: HashMap<Coord, char>,
    width: u32,
    height: u32,
}

impl Puzzle {
    fn new(width: u32, height: u32) -> Self {
        Self {
            vals: HashMap::new(),
            width,
            height,
        }
    }

    fn insert(&mut self, k: &Coord, v: char) {
        let res = self.vals.insert(k.clone(), v.clone());
        if let Some(v_old) = res {
            panic!("could not insert value {v}; already had value {v_old} for coord at {k}")
        }
    }

    fn bounds(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn get(&self, k: &Coord) -> Option<char> {
        self.vals.get(k).copied()
    }
}

fn parse_input(raw: &str) -> Puzzle {
    let rows = raw.lines().count() as u32;
    let cols = raw.lines().next().unwrap().len() as u32;

    let mut puzzle = Puzzle::new(cols, rows);

    raw.lines().enumerate().for_each(|(rownum, row)| {
        row.chars().enumerate().for_each(|(colnum, c)| {
            puzzle.insert(&Coord(colnum as u32, rownum as u32), c);
        });
    });

    puzzle
}

fn spells_xmas(coord: &Coord, dir: &Direction, letter_num: usize, puzzle: Puzzle) -> bool {
    let expected_letters = ['X', 'M', 'A', 'S'];
    let val = puzzle.get(coord).expect("coordinate not found in puzzle");
    if letter_num < 4 && val == expected_letters[letter_num] {
        if letter_num == 3 {
            // last letter, word found
            true
        } else if let Some(neighbour) = coord.neighbour(&dir, puzzle.bounds()) {
            // recurse to check neighbour in same direction for next letter
            spells_xmas(&neighbour, dir, letter_num + 1, puzzle)
        } else {
            // no neighbour found
            false
        }
    } else {
        // either not correct letter, or out of bounds
        false
    }
}

fn is_center_of_xmas_cross(coord: &Coord, puzzle: &Puzzle) -> bool {
    let c = puzzle.get(coord).expect("could not get coord");
    if c != 'A' {
        return false;
    }
    let [tl, tr, bl, br] = [
        coord
            .neighbour(
                &Direction(Some(DirectionHoriz::Left), Some(DirectionVert::Up)),
                puzzle.bounds(),
            )
            .expect("no neighbor"),
        coord
            .neighbour(
                &Direction(Some(DirectionHoriz::Right), Some(DirectionVert::Up)),
                puzzle.bounds(),
            )
            .expect("no neighbor"),
        coord
            .neighbour(
                &Direction(Some(DirectionHoriz::Left), Some(DirectionVert::Down)),
                puzzle.bounds(),
            )
            .expect("no neighbor"),
        coord
            .neighbour(
                &Direction(Some(DirectionHoriz::Right), Some(DirectionVert::Down)),
                puzzle.bounds(),
            )
            .expect("no neighbor"),
    ];

    let tl_val = puzzle.get(&tl).expect("could not get coord");
    let br_val = puzzle.get(&br).expect("could not get coord");
    if ['M', 'S'].contains(&tl_val).not() || ['M', 'S'].contains(&br_val).not() || tl_val == br_val
    {
        return false;
    }

    let tr_val = puzzle.get(&tr).expect("could not get coord");
    let bl_val = puzzle.get(&bl).expect("could not get coord");
    if ['M', 'S'].contains(&tr_val).not() || ['M', 'S'].contains(&bl_val).not() || tr_val == bl_val
    {
        return false;
    }

    return true;
}

fn part1(puzzle: Puzzle) -> usize {
    // initialize count
    let mut ct = 0;

    // loop over each coordinate
    for (coord, val) in puzzle.clone().vals {
        // if coord starts with x, start recursive word check
        if val == 'X' {
            // check in each direction
            for direction in [
                Direction(Some(DirectionHoriz::Left), None),
                Direction(Some(DirectionHoriz::Right), None),
                Direction(Some(DirectionHoriz::Left), Some(DirectionVert::Up)),
                Direction(Some(DirectionHoriz::Right), Some(DirectionVert::Up)),
                Direction(None, Some(DirectionVert::Up)),
                Direction(Some(DirectionHoriz::Left), Some(DirectionVert::Down)),
                Direction(Some(DirectionHoriz::Right), Some(DirectionVert::Down)),
                Direction(None, Some(DirectionVert::Down)),
            ] {
                if spells_xmas(&coord, &direction, 0, puzzle.clone()) {
                    ct += 1;
                }
            }
        }
    }
    ct
}

fn part2(puzzle: &Puzzle) -> usize {
    let mut ct = 0;
    let (cols, rows) = puzzle.bounds();
    for col in 1..cols - 1 {
        for row in 1..rows - 1 {
            if is_center_of_xmas_cross(&Coord(col, row), puzzle) {
                ct += 1;
            }
        }
    }

    ct
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse() {
        let parsed = parse_input(SAMPLE);

        // arbitrary sample
        assert_eq!(parsed.vals.get(&Coord(2, 2)), Some(&'X'));
        assert_eq!(parsed.vals.get(&Coord(2, 0)), Some(&'M'));
        assert_eq!(parsed.vals.get(&Coord(0, 2)), Some(&'A'));
        assert_eq!(parsed.vals.get(&Coord(9, 9)), Some(&'X'));
        assert_eq!(parsed.vals.get(&Coord(0, 10)), None); // out of bounds
        assert_eq!(parsed.vals.len(), 100);
        assert_eq!(parsed.bounds(), (10, 10));
    }

    #[test]
    fn test_part1() {
        let puzzle = parse_input(SAMPLE);

        assert_eq!(part1(puzzle), 18)
    }

    #[test]
    fn test_part2() {
        let puzzle = parse_input(SAMPLE);

        assert_eq!(part2(&puzzle), 9)
    }

    #[test]
    fn test_contains() {
        dbg!(&['M', 'S'].contains(&'S'));
    }
}
