use crate::util::board::Board;
use crate::util::board::BOARD_LENGTH;
use crate::util::position::Position;
use crate::util::cell::Cell;
use crate::util::cutout::Cutout;
use crate::util::orientation::Orientation;

use rand::distributions::Uniform;
use rand::distributions::Distribution;

#[derive(Copy, Clone, Hash, Debug)]
pub struct Stone {
    consists_of: [Position; 4],
    middle: Position,
    color: Cell,
}

impl Stone {
    // TODO: create other kinds of stones at random
    pub fn new() -> Stone {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..5);
        let middle = (BOARD_LENGTH / 2) as i32;
        let throw = die.sample(&mut rng);
        match throw {
            1 => Stone {
                consists_of: [
                    Position::new(middle, middle - 2),
                    Position::new(middle, middle - 1),
                    Position::new(middle, middle),
                    Position::new(middle, middle + 1),
                ],
                middle: Position::new(middle, middle),
                color: Cell::Blue,
            },
            2 => Stone {
                consists_of: [
                    Position::new(middle - 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle, middle),
                    Position::new(middle, middle + 1),
                ],
                middle: Position::new(middle, middle),
                color: Cell::Green,
            },
            3 => Stone {
                consists_of: [
                    Position::new(middle - 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle - 1, middle),
                    Position::new(middle, middle),
                ],
                middle: Position::new(middle, middle),
                color: Cell::Orange,
            },
            4 => Stone {
                consists_of: [
                    Position::new(middle + 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle, middle),
                    Position::new(middle, middle + 1),
                ],
                middle: Position::new(middle, middle),
                color: Cell::Purple,
            },
            5 => Stone {
                consists_of: [
                    Position::new(middle - 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle, middle),
                    Position::new(middle + 1, middle),
                ],
                middle: Position::new(middle, middle),
                color: Cell::Yellow,
            },
            i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!()
        }
    }

    pub fn color(&self) -> &Cell {
        &self.color
    }

    pub fn fall(&mut self, orientation: &Orientation) {
        for position in self.consists_of.iter_mut() {
            *position = position.get_under(orientation);
        }
        self.middle = self.middle.get_under(orientation);
    }

    pub fn positions(&self) -> &[Position; 4] {
        return &self.consists_of;
    }

    pub fn rotate_clockwise_unless_blocked(&mut self, board: &Board) -> bool {
        let cutout = Cutout::new(self.middle, 4);
        if !cutout.is_valid(BOARD_LENGTH as i32) {
            return false;
        }
        for position in self.positions().iter() {
	        let new_position = position.rotate_clockwise(cutout);
            if !new_position.is_valid(BOARD_LENGTH as i32) || !board.is_cell_free(&new_position) {
                return false;
            } else {
            }
        }
        for position in self.consists_of.iter_mut() {
            *position = position.rotate_clockwise(cutout);
        }
        return true;
    }

    pub fn rotate_counter_clockwise_unless_blocked(&mut self, board: &Board) -> bool {
        let cutout = Cutout::new(self.middle, 4);
        if !cutout.is_valid(BOARD_LENGTH as i32) {
            return false;
        }
        for position in self.positions().iter() {
	        let new_position = position.rotate_counter_clockwise(cutout);
            if !new_position.is_valid(BOARD_LENGTH as i32) || !board.is_cell_free(&new_position) {
                return false;
            } else {
            }
        }
        for position in self.consists_of.iter_mut() {
            *position = position.rotate_counter_clockwise(cutout);
        }
        return true;
    }
}
