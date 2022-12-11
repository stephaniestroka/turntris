use std::fmt;
use crate::util::cutout::Cutout;
use crate::util::orientation::Orientation;

#[derive(Copy, Clone, Eq, Hash, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x, y: y,
        }
    }

    pub fn get_index(&self, l: usize, orientation: &Orientation) -> usize {
        let x = self.x as usize;
        let y = self.y as usize;
        match orientation {
            Orientation::Zero => l * y + x,
            Orientation::Ninety => l * x + (l - y - 1),
            Orientation::OneEighty => l * (l - y - 1) + (l - x - 1),
            Orientation::TwoSeventy => l * (l - x - 1) + y,
        }
    }

    // Returns the position under the given position.
    // The term 'under' depends on the boards orientation: 0, 90, 180 or 270 degrees.
    // Note that the position might be outside of the board. Use `is_valid`
    // to check if it is on the board.
    pub fn get_under(&self, orientation: &Orientation) -> Position {
        match orientation {
            Orientation::Zero => Position::new(self.x, self.y + 1),
            Orientation::Ninety => Position::new(self.x + 1, self.y),
            Orientation::OneEighty => Position::new(self.x, self.y - 1),
            Orientation::TwoSeventy => Position::new(self.x - 1, self.y),
        }
    }

    pub fn move_to(&self, x_diff: i32, y_diff: i32) -> Position {
        Position::new(self.x + x_diff, self.y + y_diff)
    }

    // 0 0 1    0 0 0
    // 0 0 0 => 0 0 0
    // 0 0 0    0 0 1
    pub fn rotate_clockwise(&self, cutout: Cutout) -> Position {
        let x_in_cutout = self.x - cutout.get_top_left().x;
        let y_in_cutout = self.y - cutout.get_top_left().y;

        return Position::new(cutout.get_top_left().x + (cutout.size() - y_in_cutout - 1), 
                             cutout.get_top_left().y + x_in_cutout);
	}

    // 0 0 0    0 0 1
    // 0 0 0 => 0 0 0
    // 0 0 1    0 0 0
    pub fn rotate_counter_clockwise(&self, cutout: Cutout) -> Position {
        let x_in_cutout = self.x - cutout.get_top_left().x;
        let y_in_cutout = self.y - cutout.get_top_left().y;

        return Position::new(cutout.get_top_left().x + y_in_cutout, 
                             cutout.get_top_left().y + (cutout.size() - x_in_cutout - 1));
	}

    pub fn is_valid(&self, board_length: i32) -> bool {
        return match self {
            Position{ x, y } if x >= &0 && x < &board_length && y >= &0 && y < &board_length => true,
            _ => false
        }
    }
}
