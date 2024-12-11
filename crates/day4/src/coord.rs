use std::fmt::Display;

use crate::direction::{Direction, DirectionHoriz, DirectionVert};

/// X-Y coordinate, top to bottom, left to right
#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub(crate) struct Coord(pub(crate) u32, pub(crate) u32);

impl Coord {
    /// Return coordinate of
    pub(crate) fn neighbour(
        &self,
        dir: &Direction,
        // w,h
        max_bounds: (u32, u32),
    ) -> Option<Coord> {
        match dir {
            Direction(Some(DirectionHoriz::Left), _) if self.0 <= 0 => None,
            Direction(Some(DirectionHoriz::Right), _) if self.0 >= (max_bounds.0 - 1) => None,
            Direction(_, Some(DirectionVert::Up)) if self.1 <= 0 => None,
            Direction(_, Some(DirectionVert::Down)) if self.1 >= (max_bounds.1 - 1) => None,
            Direction(None, None) => panic!("cannot reference self; infinite recursion risk"), // Self reference makes no sense
            Direction(dir_h, dir_v) => {
                let new_x = match dir_h {
                    None => self.0,
                    Some(DirectionHoriz::Left) => self.0 - 1,
                    Some(DirectionHoriz::Right) => self.0 + 1,
                };

                let new_y = match dir_v {
                    None => self.1,
                    Some(DirectionVert::Up) => self.1 - 1,
                    Some(DirectionVert::Down) => self.1 + 1,
                };

                Some(Coord(new_x, new_y))
            }
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod coord_tests {
    use super::*;

    #[test_case::test_case(Coord(0,5), Direction(Some(DirectionHoriz::Left), Some(DirectionVert::Down)) => None ; "fails when requesting left neighbour from left col")]
    #[test_case::test_case(Coord(0,99), Direction(Some(DirectionHoriz::Right), Some(DirectionVert::Down)) => None; "fails when requesting right neighbour from right col")]
    #[test_case::test_case(Coord(5,0), Direction(Some(DirectionHoriz::Right), Some(DirectionVert::Up)) => None; "fails when requesting top neighbour from top row")]
    #[test_case::test_case(Coord(5,99), Direction(Some(DirectionHoriz::Right), Some(DirectionVert::Down)) => None; "fails when requesting bottom neighbour from bottom row")]
    #[test_case::test_case(Coord(5,5), Direction(Some(DirectionHoriz::Right), None) => Some(Coord(6,5)); "successfully returns right neighbor")]
    #[test_case::test_case(Coord(1,5), Direction(Some(DirectionHoriz::Left), None) => Some(Coord(0,5)); "successfully returns left neighbor")]
    #[test_case::test_case(Coord(5,5), Direction(None, Some(DirectionVert::Up)) => Some(Coord(5,4)); "successfully returns top neighbor")]
    #[test_case::test_case(Coord(5,5), Direction(None, Some(DirectionVert::Down)) => Some(Coord(5,6)); "successfully returns bottom neighbor")]
    #[test_case::test_case(Coord(5,5), Direction(Some(DirectionHoriz::Left), Some(DirectionVert::Down)) => Some(Coord(4,6)); "successfully returns diagonal neighbor")]
    fn test_neighbour_method(c: Coord, dir: Direction) -> Option<Coord> {
        c.neighbour(&dir, (100, 100))
    }

    #[test]
    #[should_panic(expected = "infinite recursion risk")]
    fn test_neighbour_method_panics_when_asked_to_return_self() {
        Coord(5, 5)
            .neighbour(&Direction(None, None), (100, 100))
            .expect("invalid coord");
    }
}
