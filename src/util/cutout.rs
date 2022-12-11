use crate::util::position::Position;

#[derive(Copy, Clone, Eq, Hash, Debug)]
pub struct Cutout {
    top_left: Position,
    size: i32,
}

impl PartialEq for Cutout {
    fn eq(&self, other: &Cutout) -> bool {
        return self.top_left == other.top_left && self.size == other.size;
    }
}

impl Cutout {
    pub fn new(middle: Position, size: i32) -> Cutout {
        Cutout {
            top_left: middle.move_to(-size/2, -size/2),
            size: size,
        }
    }

    pub fn get_top_left(&self) -> Position {
        self.top_left
    }
    
    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn is_valid(&self, board_length: i32) -> bool {
        return self.top_left.is_valid(board_length);
    }
}