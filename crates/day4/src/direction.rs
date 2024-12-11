use std::fmt::Display;

pub(crate) enum DirectionVert {
    Up,
    Down,
}

impl Display for DirectionVert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "up",
                Self::Down => "down",
            }
        )
    }
}

pub(crate) enum DirectionHoriz {
    Right,
    Left,
}

impl Display for DirectionHoriz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => "left",
                Self::Right => "right",
            }
        )
    }
}

pub(crate) struct Direction(
    pub(crate) Option<DirectionHoriz>,
    pub(crate) Option<DirectionVert>,
);

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            if let Some(d) = &self.0 {
                d.to_string()
            } else {
                "-".to_owned()
            },
            if let Some(d) = &self.1 {
                d.to_string()
            } else {
                "-".to_owned()
            }
        )
    }
}
