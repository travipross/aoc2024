// For parellel iterators
use rayon::prelude::*;

use std::{
    collections::HashMap,
    error::Error,
    io::{self, Write},
    sync::{Arc, Mutex},
};

const RAW: &str = include_str!("../input.txt");
type Coord = (i32, i32);
type OnGrid = bool;

#[derive(Debug, Default, PartialEq, Clone)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone)]
struct Status {
    is_wall: bool,
    visited_dir: Option<Direction>,
    potential_obstruction: bool,
}

#[derive(Debug, Default, Clone)]
struct Guard {
    pos: Coord,
    dir: Direction,
}

#[derive(Debug, Clone)]
struct Input {
    map: HashMap<Coord, Status>,
    guard: Guard,
}

impl Input {
    fn from_str(raw: &str) -> Self {
        let mut input = Input {
            guard: Guard::default(),
            map: HashMap::new(),
        };
        for (row, line) in raw.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                let coord = (row as i32, col as i32);
                match char {
                    '#' => {
                        input.map.insert(
                            coord,
                            Status {
                                is_wall: true,
                                visited_dir: None,
                                potential_obstruction: false,
                            },
                        );
                    }
                    '.' => {
                        input.map.insert(
                            coord,
                            Status {
                                is_wall: false,
                                visited_dir: None,
                                potential_obstruction: false,
                            },
                        );
                    }
                    '^' => {
                        input.map.insert(
                            coord,
                            Status {
                                is_wall: false,
                                visited_dir: Some(Direction::Up),
                                potential_obstruction: false,
                            },
                        );
                        input.guard.pos = coord;
                    }
                    _ => {
                        panic!("unrecognized char")
                    }
                }
            }
        }
        input
    }

    fn next_dir(&self) -> Direction {
        match self.guard.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn(&mut self) {
        self.guard.dir = self.next_dir();

        // println!("turning {:?}", self.guard.dir)
    }

    fn next_pos(&self, start_coord: &Coord, dir: &Direction) -> Coord {
        match dir {
            Direction::Up => (start_coord.0 - 1, start_coord.1),
            Direction::Right => (start_coord.0, start_coord.1 + 1),
            Direction::Down => (start_coord.0 + 1, start_coord.1),
            Direction::Left => (start_coord.0, start_coord.1 - 1),
        }
    }

    fn next_guard_pos(&self) -> Coord {
        self.next_pos(&self.guard.pos, &self.guard.dir)
    }

    fn next_traveled_coord_in_same_dir(&self, dir: &Direction) -> Option<Coord> {
        let mut travelled_coord = None;

        // first coord to check is the immediate next position if the guard were to turn to next dir
        let mut coord_to_check = self.next_pos(&self.guard.pos, &self.next_dir());

        // loop while coord to check is on the grid
        while let Some(val) = self.map.get(&coord_to_check) {
            // if the coord has been visited, we'll check the direction
            if let Some(visited_dir) = &val.visited_dir {
                // if the checked coordinate has been visited in the direction we're looking,
                // we note that as a potential obstruction point and break
                if visited_dir == dir {
                    travelled_coord = Some(coord_to_check);
                    break;
                }
            // if the coord is a wall, we're done looking
            } else if val.is_wall {
                break;
            }

            // set the next coord in the same direction to check on the next loop
            coord_to_check = self.next_pos(&coord_to_check, dir);
        }

        travelled_coord
    }

    fn step(&mut self) {
        self.guard.pos = self.next_guard_pos();
        self.map
            .get_mut(&self.guard.pos)
            .expect("could not get next coord")
            .visited_dir = Some(self.guard.dir.clone());
        // println!("stepping {:?} to {:?}", self.guard.dir, self.guard.pos)
    }

    // walks the guard until a wall is in front of him. If a loop is detected, an error is returned
    fn walk_to_wall(&mut self) -> Result<OnGrid, Box<dyn Error>> {
        let mut on_grid = true;
        loop {
            // If the next position in the next dir has already been travelled in that dir, then we have
            // potential for a never-ending loop. Save potential obstruction at next position in current dir
            if let Some(_c) = self.next_traveled_coord_in_same_dir(&self.next_dir()) {
                self.map
                    .get_mut(&self.next_guard_pos())
                    .expect("could not get next pos")
                    .potential_obstruction = true;
            }

            // if next position in current dir is Some(), then we're on grid
            if let Some(coord) = self.map.get(&self.next_guard_pos()) {
                // if next position is a wall, then we've finished our walk
                if coord.is_wall {
                    break;
                } else if let Some(dir) = &coord.visited_dir {
                    if dir == &self.guard.dir {
                        return Err("already visited this grid in this direction".into());
                    } else {
                        self.step();
                        continue;
                    }
                // otherwise, we step and restart the loop
                } else {
                    self.step();
                    continue;
                }
            // if not on grid, we exit early
            } else {
                on_grid = false;
                break;
            };
        }
        Ok(on_grid)
    }

    fn walk_to_end(&mut self) -> Result<usize, Box<dyn Error>> {
        loop {
            match self.walk_to_wall() {
                // If still on the grid, we turn and loop again
                Ok(on_grid) if on_grid => {
                    self.turn();
                }
                // if error, this indicates an infinite loop, triggering early termination
                Err(_) => return Err("infinite loop detected".into()),
                // if not on grid, we're done looping and can count walked paths
                _ => {
                    break;
                }
            }
        }

        let ct = self
            .map
            .values()
            .filter(|&s| s.visited_dir.is_some())
            .count();

        Ok(ct)
    }
}

fn parse_input(raw: &str) -> Input {
    Input::from_str(raw)
}

fn part1(input: &Input) -> usize {
    let mut input = input.clone();

    input.walk_to_end().expect("infinite loop detected")
}

fn part2(input: &Input) -> usize {
    let input = input.clone();
    let obstacles_causing_loop = Arc::new(Mutex::new(0_usize));

    // for each blank coord, try swapping with wall and running until loop detected
    input
        .map
        .par_iter()
        .filter(|(c, s)| !s.is_wall && **c != input.guard.pos)
        .for_each(|(new_obst_coord, _)| {
            // println!("checking obstacle at {:?}", &new_obst_coord);
            let mut temp_input = input.clone();
            temp_input
                .map
                .get_mut(&new_obst_coord)
                .expect("could not get coord")
                .is_wall = true;

            if temp_input.walk_to_end().is_err() {
                print!(".");
                io::stdout().flush().expect("could not flush");
                let mut lock = obstacles_causing_loop
                    .lock()
                    .expect("could not acquire lock");
                *lock += 1;
            }
        });
    println!("");

    let ct = obstacles_causing_loop
        .lock()
        .expect("could not acquire lock")
        .clone();
    ct
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_input() {
        let parsed = parse_input(SAMPLE);
        assert_eq!(parsed.map[&(3, 2)].is_wall, true);
        assert_eq!(parsed.map[&(3, 3)].is_wall, false);
        assert_eq!(parsed.map[&(0, 4)].is_wall, true);
        assert_eq!(parsed.map[&(6, 4)].is_wall, false);
        assert_eq!(parsed.guard.pos, (6, 4));
        assert_eq!(parsed.guard.dir, Direction::Up);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = 41;

        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SAMPLE);
        let expected = 6;

        assert_eq!(part2(&input), expected);
    }
}
