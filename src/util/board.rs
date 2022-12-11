use crate::util::cell::Cell;

use crate::util::orientation::Orientation;
use crate::util::position::Position;
use crate::util::stone::Stone;

pub const BOARD_LENGTH: usize = 30;


pub struct Board {
    cells: [Cell; BOARD_LENGTH * BOARD_LENGTH],
    orientation: Orientation,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::Free; BOARD_LENGTH * BOARD_LENGTH],
            orientation: Orientation::Zero
        }
    }

    pub fn get_orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn is_cell_free(&self, position: &Position) -> bool {
        let i = position.get_index(BOARD_LENGTH, &self.orientation);
        return self.cells[i] == Cell::Free;
    }

    // Returns false if the cell under the given stone is free.
    pub fn is_blocked_below(&self, stone: &Stone) -> bool {
        for position in stone.positions().iter() {
            let position_under = position.get_under(&self.orientation);
            if !position_under.is_valid(BOARD_LENGTH as i32) || !self.is_cell_free(&position_under) {
                return true;
            }
        }
        return false;
    }

    // Cells with the current fixed stones on the board.
    pub fn get_cells(&self) -> [Cell; BOARD_LENGTH * BOARD_LENGTH] {
        return self.cells;
    }
    
    pub fn maybe_delete_rows(&mut self) {
        let mut del_rows = 0;
        for y in 0..BOARD_LENGTH {
            let mut row_complete = true;
            for x in 0..BOARD_LENGTH {
                if self.cells[y * BOARD_LENGTH + x] == Cell::Free {
                    row_complete = false;
                    continue
                }
            }
            if row_complete {
                del_rows += 1;
            }
        }
        if del_rows > 0 {
            for y in (del_rows..BOARD_LENGTH).rev() {
                for x in 0..BOARD_LENGTH {
                    self.cells[y * BOARD_LENGTH + x] = self.cells[(y - del_rows) * BOARD_LENGTH + x];
                }
            }
            for y in 0..del_rows {
                for x in 0..BOARD_LENGTH {
                    self.cells[y * BOARD_LENGTH + x] = Cell::Free;
                }
            }
        }
    }

    pub fn add(&mut self, stone: &Stone) {
         for position in stone.positions().iter() {
            let i = position.get_index(BOARD_LENGTH, &self.orientation);    
            self.cells[i] = *stone.color();
        }
    }

    pub fn rotate_clockwise(&mut self) {
        let mut rotated_cells = [Cell::Free; BOARD_LENGTH * BOARD_LENGTH];
        for y in 0..BOARD_LENGTH {
            for x in 0..BOARD_LENGTH {
                let new_x = BOARD_LENGTH - y - 1;
                let new_y = x;
                rotated_cells[new_y * BOARD_LENGTH + new_x] = self.cells[y * BOARD_LENGTH + x];
            }
        }
        self.cells = rotated_cells;
        match self.orientation {
            Orientation::Zero => self.orientation = Orientation::Ninety,
            Orientation::Ninety => self.orientation = Orientation::OneEighty,
            Orientation::OneEighty => self.orientation = Orientation::TwoSeventy,
            Orientation::TwoSeventy => self.orientation = Orientation::Zero,
        }

    }

    pub fn rotate_counter_clockwise(&mut self) {
        let mut rotated_cells = [Cell::Free; BOARD_LENGTH * BOARD_LENGTH];
        for y in 0..BOARD_LENGTH {
            for x in 0..BOARD_LENGTH {
                let new_x = y;
                let new_y = BOARD_LENGTH - x - 1;
                rotated_cells[new_y * BOARD_LENGTH + new_x] = self.cells[y * BOARD_LENGTH + x];
            }
        }
        self.cells = rotated_cells;
        match self.orientation {
            Orientation::Zero => self.orientation = Orientation::TwoSeventy,
            Orientation::Ninety => self.orientation = Orientation::Zero,
            Orientation::OneEighty => self.orientation = Orientation::Ninety,
            Orientation::TwoSeventy => self.orientation = Orientation::OneEighty,
        }
    }
}
