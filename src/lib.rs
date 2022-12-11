use crate::util::cell::Cell;
use crate::util::board::Board;
use crate::util::stone::Stone;
use crate::util::board::BOARD_LENGTH;

mod util;

use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Game {
    falling_stone:  Option<Stone>,
    board: Board,
    cells: [Cell; BOARD_LENGTH * BOARD_LENGTH],
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            falling_stone: None,
            cells: [Cell::Free; BOARD_LENGTH * BOARD_LENGTH],
        }
    }

    fn add_stone(&mut self) -> bool {
        let new_stone = Stone::new();
        for position in new_stone.positions().iter() {
            if !self.board.is_cell_free(position) {
                return false;
            }
        }
        self.falling_stone = Some(new_stone);
        true
    }

    fn stone_falls(&mut self) -> bool {
        return match &mut self.falling_stone {
            None => {
                false
            }
            Some(stone) => {
                if self.board.is_blocked_below(stone) {
                    return false;
                }
                stone.fall(&self.board.get_orientation());
                true
            }
        }
    }

    pub fn drop_stone(&mut self) {
        return match &mut self.falling_stone {
            None => {
            }
            Some(stone) => {
                while !self.board.is_blocked_below(stone) {
                    stone.fall(&self.board.get_orientation());
                }
            }
        }
    }

    // Snapshot of the board with fixed stones and falling stone.
    pub fn snapshot(&mut self) -> *const Cell {
        return match &self.falling_stone {
            None => {
                self.cells = self.board.get_cells();
                self.cells.as_ptr()
            }
            Some(stone) => {
                self.cells = self.board.get_cells();
                for position in stone.positions().iter() {
                    let i = position.get_index(BOARD_LENGTH, self.board.get_orientation());    
                    self.cells[i] = *stone.color();
                }
                self.cells.as_ptr()
            }
        }
    }

    // The length of one side of the canvas.
    pub fn length(&self) -> usize {
        return BOARD_LENGTH;
    }

    // Advances the game by one tick.
    pub fn tick(&mut self) -> bool {
        // States:
        // - The free falling stone will move down.
        // - If a row is full, the row disappears and all stones will fall down.
        // - If no stone is falling, create a new stone.
        if !self.stone_falls() {
            match &self.falling_stone {
                None => {}
                Some(stone) => {
                    self.board.add(stone);
                }
            }
            self.board.maybe_delete_rows();
            return self.add_stone();
        }
        return true;
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.board.rotate_counter_clockwise();
    }

    pub fn rotate_clockwise(&mut self) {
        self.board.rotate_clockwise();
    }

    pub fn rotate_stone_clockwise(&mut self) -> bool {
		return match &mut self.falling_stone {
            None => {
                false
            }
            Some(stone) => {
                return stone.rotate_clockwise_unless_blocked(&self.board);
            }
		}
	}

    pub fn rotate_stone_counter_clockwise(&mut self) -> bool {
		return match &mut self.falling_stone {
            None => {
                false
            }
            Some(stone) => {
                return stone.rotate_counter_clockwise_unless_blocked(&self.board);
            }
		}
	}
}
