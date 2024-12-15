use std::{fmt::Display, iter::repeat_n};

const RAW: &str = include_str!("../input.txt");
type Input = Vec<usize>;

enum Segment {
    File { id: usize, size: usize },
    Empty { size: usize },
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::File { id, size } => String::from_iter(repeat_n(id.to_string(), *size)),
                Self::Empty { size } => String::from_iter(repeat_n(".", *size)),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DefragMode {
    Part,
    Whole,
}

type DiskMap = Vec<Segment>;

fn parse_input(raw: &str) -> Input {
    // Assume one line
    raw.chars()
        .map(|c| {
            c.to_string()
                .parse::<usize>()
                .expect("could not parse char as usize")
        })
        .collect::<Vec<_>>()
}

fn create_disk_map(input: Input) -> DiskMap {
    let mut map = DiskMap::with_capacity(input.len());
    let mut file_id = 0;
    for (i, n) in input.iter().enumerate() {
        if i % 2 == 0 {
            map.push(Segment::File {
                id: file_id,
                size: *n,
            });
            file_id += 1;
        } else {
            map.push(Segment::Empty { size: *n });
        }
    }
    map
}

fn defrag_disk_map(src_disk: &mut DiskMap, defrag_mode: &DefragMode) {
    match defrag_mode {
        DefragMode::Part => {
            let mut dest_disk = DiskMap::new();
            let mut write_idx = 0;
            while write_idx < src_disk.len() {
                let seg = &src_disk[write_idx];
                match *seg {
                    Segment::File { id, size } => dest_disk.push(Segment::File { id, size }),
                    Segment::Empty { size: freesize } => {
                        fill_freespace(src_disk, freesize, write_idx, &mut dest_disk);
                    }
                }
                write_idx += 1;
            }
            *src_disk = dest_disk;
        }
        DefragMode::Whole => {
            // Read from right side of src disk for files to defrag
            let mut read_idx = src_disk.len() - 1;

            while read_idx > 0 {
                // if file, look for empty space
                if let Segment::File { id, size: filesize } = src_disk[read_idx] {
                    // scan from left for empty space
                    let mut write_idx = 0;

                    // loop until reaching index of file
                    while write_idx < read_idx {
                        // if segment is empty, check if it has enough space to store file
                        if let Segment::Empty { size: freesize } = src_disk[write_idx] {
                            // if file fits in empty space, put it there
                            if freesize >= filesize {
                                src_disk[read_idx] = Segment::Empty { size: filesize };
                                src_disk[write_idx] = Segment::File { id, size: filesize };

                                // special case: space left over
                                if freesize >= filesize {
                                    src_disk.insert(
                                        write_idx + 1,
                                        Segment::Empty {
                                            size: freesize - filesize,
                                        },
                                    );
                                }
                                // stop loop after file moved
                                break;
                            }
                        }
                        // prepare to look at next segment on next iteration
                        write_idx += 1;
                    }
                }
                // move left to next file
                read_idx -= 1;
            }
        }
    }
}

// Read from right side of src disk
// Fit any file in current write_idx as possible
// stop when read index crosses write index
fn fill_freespace(
    src_disk: &mut DiskMap,
    mut freesize: usize,
    write_idx: usize,
    dest_disk: &mut DiskMap,
) {
    let mut read_idx = src_disk.len() - 1;
    while freesize > 0 && read_idx > write_idx {
        // if last segment is a file
        if let Segment::File { id, size: filesize } = src_disk[read_idx] {
            // if entire file fits in empty space, move it there
            if filesize <= freesize {
                dest_disk.push(Segment::File { id, size: filesize });
                freesize -= filesize;
                src_disk.remove(read_idx);
                read_idx -= 1;
            } else {
                // partial file fits in space
                dest_disk.push(Segment::File { id, size: freesize });
                src_disk[read_idx] = Segment::File {
                    id,
                    size: filesize - freesize,
                };
                freesize = 0;
            }
        } else {
            read_idx -= 1;
        }
    }
}

fn calculate_checksum(map: &DiskMap) -> usize {
    let mut idx = 0;
    let mut checksum = 0;
    for f in map {
        match f {
            Segment::File { id, size } => {
                for _ in 0..*size {
                    checksum += id * idx;
                    idx += 1;
                }
            }
            Segment::Empty { size } => {
                for _ in 0..*size {
                    idx += 1;
                }
            }
        }
    }
    checksum
}

fn part1(input: &Input) -> usize {
    let mut map = create_disk_map(input.clone());
    defrag_disk_map(&mut map, &DefragMode::Part);
    calculate_checksum(&map)
}

fn part2(input: &Input) -> usize {
    let mut map = create_disk_map(input.clone());
    defrag_disk_map(&mut map, &DefragMode::Whole);
    calculate_checksum(&map)
}

fn main() {
    let input = parse_input(RAW);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    fn render_disk_map(map: &DiskMap) -> String {
        String::from_iter(map.iter().map(ToString::to_string))
    }

    #[test]
    fn test_parse_input() {
        let input = parse_input(SAMPLE);
        let expected = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
        assert_eq!(input, expected);
    }

    #[test_case::case("12345" => "0..111....22222".to_owned())]
    #[test_case::case("101010101" => "01234".to_owned())]
    #[test_case::case("10101010101010101010" => "0123456789".to_owned())]
    #[test_case::case("1010101010101010101010101" => "0123456789101112".to_owned())]
    fn test_render_disk_map(raw: &str) -> String {
        let input = parse_input(raw);
        let disk_map = create_disk_map(input);
        render_disk_map(&disk_map)
    }

    #[test]
    fn test_defrag_map() {
        let input = parse_input("12345");
        let mut disk_map = create_disk_map(input);
        defrag_disk_map(&mut disk_map, &DefragMode::Part);
        assert_eq!(render_disk_map(&disk_map), "022111222".to_owned())
    }

    #[test]
    fn test_swap_with_slice() {
        let mut my_vec = vec![1, 1, 1, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 3, 3, 3];
        let expected = vec![1, 1, 1, 3, 3, 3, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0];
        let gap_idx = 3;
        let file_idx = 13;
        let len = 3;

        let (left, right) = my_vec.split_at_mut(file_idx);
        left[gap_idx..gap_idx + len].swap_with_slice(&mut right[0..len]);
        assert_eq!(my_vec, expected);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(SAMPLE);
        let expected = 1928;

        assert_eq!(part1(&input), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SAMPLE);
        let expected = 2858;

        assert_eq!(part2(&input), expected);
    }
}
