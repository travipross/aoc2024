use std::collections::HashMap;

use itertools::Itertools;

const RAW: &str = include_str!("../input.txt");
type Row = i32;
type Col = i32;
type Coord = (Row, Col);

type AntennaFreq = char;

#[derive(Debug, Clone, PartialEq)]
enum AntinodeModel {
    Old,
    New,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    freq: Option<AntennaFreq>,
    is_antinode: bool,
}

#[derive(Debug, Clone)]
struct Input {
    map: HashMap<Coord, Location>,
    bounds: (usize, usize),
}

impl Input {
    fn from_str(raw: &str) -> Self {
        let mut input = Input {
            map: HashMap::new(),
            bounds: (0, 0),
        };
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in raw.lines().enumerate() {
            if row > max_row {
                max_row = row;
            }
            for (col, char) in line.chars().enumerate() {
                if col > max_col {
                    max_col = col;
                }
                let freq = if char == '.' { None } else { Some(char) };
                input.map.insert(
                    (row as i32, col as i32),
                    Location {
                        freq,
                        is_antinode: false,
                    },
                );
            }
        }

        input.bounds = (max_row + 1, max_col + 1);
        input
    }

    fn find_antennas(&self, freq: char) -> Vec<Coord> {
        self.map
            .iter()
            .filter_map(|(c, l)| {
                if l.freq == Some(freq) {
                    Some(c.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn mark_all_antinodes(&mut self, model: &AntinodeModel) {
        // get unique frequencies found on map
        let unique_freqs = self
            .map
            .values()
            .filter_map(|l| l.freq)
            .unique()
            .collect::<Vec<_>>();

        // loop over each unique frequency
        for f in unique_freqs {
            // find antennas on map having given frequency
            let antennas = self.find_antennas(f);
            // if more than one, antinodes are possible
            if antennas.len() > 1 {
                // loop over each combination of two antenna coords
                for antenna_combos in antennas.iter().combinations(2) {
                    // find antinode locations for antennas
                    let antinodes = antinode_locations(
                        *antenna_combos[0],
                        *antenna_combos[1],
                        self.bounds,
                        model,
                    );

                    // for each antinode location, if on grid, update map location to set status
                    for antinode in antinodes {
                        self.map
                            .get_mut(&antinode)
                            .expect("could not get coord")
                            .is_antinode = true;
                    }
                }
            }
        }
    }

    fn num_antinodes(&self) -> usize {
        self.map.iter().filter(|(_, l)| l.is_antinode).count()
    }
}

fn add_coord(c: Coord, diff: Coord, invert: bool) -> Coord {
    if invert {
        (c.0 - diff.0, c.1 - diff.1)
    } else {
        (c.0 + diff.0, c.1 + diff.1)
    }
}

fn antinode_locations(
    c1: Coord,
    c2: Coord,
    bounds: (usize, usize),
    model: &AntinodeModel,
) -> Vec<Coord> {
    let diff = (c2.0 - c1.0, c2.1 - c1.1);
    let mut antinodes = vec![];
    if model == &AntinodeModel::Old {
        let antinode1 = add_coord(c2, diff, false);
        let antinode2 = add_coord(c1, diff, true);

        for n in [antinode1, antinode2] {
            if coord_in_bound(n, bounds) {
                antinodes.push(n);
            }
        }
    } else {
        // Antennas are always antinodes
        antinodes.push(c1);
        antinodes.push(c2);

        // Step in both directions until off the grid
        let mut c = add_coord(c2, diff, false);
        while coord_in_bound(c, bounds) {
            antinodes.push(c);
            c = add_coord(c, diff, false);
        }
        c = add_coord(c1, diff, true);
        while coord_in_bound(c, bounds) {
            antinodes.push(c);
            c = add_coord(c, diff, true)
        }
    }
    antinodes
}

fn coord_in_bound(c: Coord, bounds: (usize, usize)) -> bool {
    c.0 >= 0 && c.0 < bounds.0 as i32 && c.1 >= 0 && c.1 < bounds.1 as i32
}

fn parse_input(raw: &str) -> Input {
    Input::from_str(raw)
}

fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    input.mark_all_antinodes(&AntinodeModel::Old);
    input.num_antinodes()
}

fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    input.mark_all_antinodes(&AntinodeModel::New);
    input.num_antinodes()
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod day8_tests {
    use std::{collections::HashSet, hash::RandomState};

    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_input() {
        let input = parse_input(SAMPLE);
        assert_eq!(
            input.map.get(&(1, 8)).expect("does not exist").freq,
            Some('0')
        );
        assert_eq!(
            input.map.get(&(5, 6)).expect("does not exist").freq,
            Some('A')
        );
        assert_eq!(input.map.get(&(5, 7)).expect("does not exist").freq, None);
    }

    #[test_case::test_case((3, 4), (5, 5), (12, 12), vec![(7, 6), (1, 3)], AntinodeModel::Old ; "antinodes both on grid")]
    #[test_case::test_case((4,8), (5, 5), (10, 10), vec![(6,2)], AntinodeModel::Old ; "one antinode off grid")]
    #[test_case::test_case((9,8), (8,9), (10, 10), vec![], AntinodeModel::Old ; "two antinodes off grid")]
    fn test_antinode_locations(
        c1: Coord,
        c2: Coord,
        bounds: (usize, usize),
        expected_antinodes: Vec<Coord>,
        model: AntinodeModel,
    ) {
        let antinodes = antinode_locations(c1, c2, bounds, &model);

        let antinodes = HashSet::<_, RandomState>::from_iter(antinodes);
        let expected = HashSet::<_, RandomState>::from_iter(expected_antinodes);

        assert_eq!(antinodes, expected);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = 14;

        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SAMPLE);
        let expected = 34;

        assert_eq!(part2(&input), expected);
    }
}
