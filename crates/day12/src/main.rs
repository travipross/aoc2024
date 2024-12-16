use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

const RAW: &str = include_str!("../input.txt");
type Coord = (i32, i32);
type NumNeighbours = usize;

struct Region {
    crop: char,
    coords: HashMap<Coord, NumNeighbours>,
}

impl Region {
    fn perimeter(&self) -> usize {
        // for each coord, it adds n perimeters where n = 4-(similar_neighbours)
        self.coords.iter().map(|(_, n)| 4 - n).sum()
    }

    fn area(&self) -> usize {
        self.coords.len()
    }

    fn fence_cost(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn new_fence_cost(&self) -> usize {
        self.area() * self.sides()
    }

    fn sides(&self) -> usize {
        todo!()
        // walk perimeter?
    }
}

impl Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Region of {crop} plants with price {area} * {perimeter} = {price}",
            crop = self.crop,
            area = self.area(),
            perimeter = self.perimeter(),
            price = self.fence_cost()
        )
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Input {
    plot_map: HashMap<Coord, char>,
}

impl Input {
    fn regions(&self) -> Vec<Region> {
        let mut identified_coords = HashSet::new();
        let mut regions = vec![];
        for (coord, crop) in &self.plot_map {
            // if we haven't already placed this coord in a region, check it
            if !identified_coords.contains(coord) {
                let mut similar_neighbours = HashMap::new();
                self.recursive_similar_neighbours(*coord, &mut similar_neighbours);
                for (c, _) in similar_neighbours.iter() {
                    identified_coords.insert(c.clone());
                }
                let region = Region {
                    crop: *crop,
                    coords: similar_neighbours,
                };
                regions.push(region);
            }
        }
        regions
    }

    fn neighbour_coord(&self, c: Coord, dir: &Direction) -> Option<Coord> {
        match dir {
            Direction::Up => self.plot_map.get_key_value(&(c.0 - 1, c.1)),
            Direction::Right => self.plot_map.get_key_value(&(c.0, c.1 + 1)),
            Direction::Down => self.plot_map.get_key_value(&(c.0 + 1, c.1)),
            Direction::Left => self.plot_map.get_key_value(&(c.0, c.1 - 1)),
        }
        .map(|(k, _)| k.clone())
    }

    fn all_neighbours(&self, c: Coord) -> Vec<Coord> {
        let mut neighbours = vec![];
        for dir in [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ] {
            if let Some(neighbour) = self.neighbour_coord(c, &dir) {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    fn recursive_similar_neighbours(
        &self,
        coord: Coord,
        similar_neighbours: &mut HashMap<Coord, NumNeighbours>,
    ) {
        let crop = self.plot_map[&coord];
        let neighbours_with_same_crop = self
            .all_neighbours(coord)
            .into_iter()
            .filter(|&neighbour| self.plot_map[&neighbour] == crop)
            .collect::<Vec<_>>();

        // TODO: filter
        // !similar_neighbours.contains_key(&neighbour)
        // println!(
        //     "similar neighbours to {:?}: {:?}",
        //     coord, neighbours_with_same_crop
        // );
        similar_neighbours.insert(coord, neighbours_with_same_crop.len());

        // recurse
        for neighbour in neighbours_with_same_crop {
            if !similar_neighbours.contains_key(&neighbour) {
                self.recursive_similar_neighbours(neighbour, similar_neighbours);
            }
        }
    }
}

fn parse_input(raw: &str) -> Input {
    let mut input = Input {
        plot_map: HashMap::new(),
    };

    for (row, line) in raw.lines().enumerate() {
        for (col, crop) in line.char_indices() {
            input.plot_map.insert((row as i32, col as i32), crop);
        }
    }
    input
}

fn part1(input: &Input) -> usize {
    input.regions().iter().map(|r| r.fence_cost()).sum()
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
mod day12_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse_input() {
        let input = parse_input(SAMPLE);
        assert_eq!(input.plot_map.len(), 100)
    }

    #[test]
    fn test_regions() {
        let input = parse_input(SAMPLE);
        let regions = input.regions();
        // for r in regions.iter() {
        //     dbg!(r);
        // }
        assert_eq!(regions.len(), 11);
    }

    #[test]
    fn test_similar_neighbours() {
        let input = parse_input(
            r#"RRRR.
RRRR.
..RRR
..R..
"#,
        );
        assert_eq!(input.plot_map.len(), 20);
        let mut similar_neighbours = HashMap::new();
        input.recursive_similar_neighbours((0, 0), &mut similar_neighbours);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = 1930;

        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SAMPLE);
        let expected = 1206;

        assert_eq!(part2(&input), expected);
    }
}
