use std::collections::HashMap;

const RAW: &str = include_str!("../input.txt");
type Coord = (i32, i32);

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
    visited: bool,
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
                                visited: false,
                            },
                        );
                    }
                    '.' => {
                        input.map.insert(
                            coord,
                            Status {
                                is_wall: false,
                                visited: false,
                            },
                        );
                    }
                    '^' => {
                        input.map.insert(
                            coord,
                            Status {
                                is_wall: false,
                                visited: true,
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

    fn turn(&mut self) {
        self.guard.dir = match self.guard.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };

        // println!("turning {:?}", self.guard.dir)
    }

    fn next_pos(&self) -> Coord {
        match self.guard.dir {
            Direction::Up => (self.guard.pos.0 - 1, self.guard.pos.1),
            Direction::Right => (self.guard.pos.0, self.guard.pos.1 + 1),
            Direction::Down => (self.guard.pos.0 + 1, self.guard.pos.1),
            Direction::Left => (self.guard.pos.0, self.guard.pos.1 - 1),
        }
    }

    fn step(&mut self) {
        self.guard.pos = self.next_pos();
        self.map
            .get_mut(&self.guard.pos)
            .expect("could not get next coord")
            .visited = true;
        // println!("stepping {:?} to {:?}", self.guard.dir, self.guard.pos)
    }

    fn walk_to_wall(&mut self) -> bool {
        let mut on_grid = true;
        loop {
            if let Some(coord) = self.map.get(&self.next_pos()) {
                if coord.is_wall {
                    break;
                } else {
                    self.step();
                }
            } else {
                on_grid = false;
                break;
            };
        }
        on_grid
    }
}

fn parse_input(raw: &str) -> Input {
    Input::from_str(raw)
}

fn part1(input: &Input) -> usize {
    let mut input = input.clone();
    while input.walk_to_wall() {
        input.turn();
    }

    input.map.values().filter(|&s| s.visited).count()
}

fn part2(input: &Input) -> usize {
    todo!()
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    // println!("Part 2: {}", part2(&input));
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
        let expected = todo!();

        assert_eq!(part2(&input), expected);
    }
}
