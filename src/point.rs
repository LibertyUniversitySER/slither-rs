use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self {x, y}
    }

    pub fn transform(&self, direction: Direction, times: u16) -> Self {
        let times = times as i16;
        let change = match direction {
            Direction::Up => (0, -times),
            Direction::Right => (times, 0),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0)
        };

        Self::new(
            Self::transform_value(self.x, change.0),
            Self::transform_value(self.y, change.1)
        )
    }

    fn transform_value(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!("Transform value {} by {} will result in a negative value", value, by);
        } else {
            (value as i16 + by) as u16
        }
    }
}