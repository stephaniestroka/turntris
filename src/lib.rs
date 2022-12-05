mod utils;

use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use std::fmt;
use rand::distributions::Uniform;
use rand::distributions::Distribution;

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

const BOARD_LENGTH: usize = 30;

enum Orientation {
    Zero, Ninety, OneEighty, TwoSeventy,
}

#[derive(Copy, Clone, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {
            x: x, y: y,
        }
    }

    fn get_index(&self, orientation: &Orientation) -> usize {
        let l = BOARD_LENGTH as usize;
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
    fn get_under(&self, orientation: &Orientation) -> Position {
        match orientation {
            Orientation::Zero => Position::new(self.x, self.y + 1),
            Orientation::Ninety => Position::new(self.x + 1, self.y),
            Orientation::OneEighty => Position::new(self.x, self.y - 1),
            Orientation::TwoSeventy => Position::new(self.x - 1, self.y),
        }
    }

    fn get_left(&self, orientation: &Orientation) -> Position {
		match orientation {
            Orientation::Zero => Position::new(self.x - 1, self.y),
            Orientation::Ninety => Position::new(self.x, self.y + 1),
            Orientation::OneEighty => Position::new(self.x + 1, self.y),
            Orientation::TwoSeventy => Position::new(self.x, self.y - 1),
        }
	}

    fn get_right(&self, orientation: &Orientation) -> Position {
		match orientation {
            Orientation::Zero => Position::new(self.x + 1, self.y),
            Orientation::Ninety => Position::new(self.x, self.y - 1),
            Orientation::OneEighty => Position::new(self.x - 1, self.y),
            Orientation::TwoSeventy => Position::new(self.x, self.y + 1),
        }
	}

    fn is_valid(&self) -> bool {
        return match self {
            Position{ x, y } if x >= &0 && x < &(BOARD_LENGTH as i32) && y >= &0 && y < &(BOARD_LENGTH as i32) => true,
            _ => false
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


//#[derive(Clone, Copy)]
#[derive(Copy, Clone, Hash, Debug)]
pub struct Stone {
    consists_of: [Position; 4],
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
                color: Cell::Blue,
            },
            2 => Stone {
                consists_of: [
                    Position::new(middle - 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle, middle),
                    Position::new(middle, middle + 1),
                ],
                color: Cell::Green,
            },
            3 => Stone {
                consists_of: [
                    Position::new(middle - 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle - 1, middle),
                    Position::new(middle, middle),
                ],
                color: Cell::Orange,
            },
            4 => Stone {
                consists_of: [
                    Position::new(middle + 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle, middle),
                    Position::new(middle, middle + 1),
                ],
                color: Cell::Purple,
            },
            5 => Stone {
                consists_of: [
                    Position::new(middle - 1, middle - 1),
                    Position::new(middle, middle - 1),
                    Position::new(middle, middle),
                    Position::new(middle + 1, middle),
                ],
                color: Cell::Yellow,
            },
            i32::MIN..=0_i32 | 5_i32..=i32::MAX => todo!()
        }
    }

    fn color(&self) -> &Cell {
        &self.color
    }
    
    fn mut_positions(&mut self) -> &mut [Position; 4] {
        return &mut self.consists_of;
    }

    fn positions(&self) -> &[Position; 4] {
        return &self.consists_of;
    }

    fn move_unless_blocked(&mut self, direction: Direction, environment: &BoardEnvironment) -> bool {
        for position in self.positions().iter() {
	        let new_position = match direction {
		        Direction::Left => position.get_left(&environment.orientation),
		        Direction::Right => position.get_right(&environment.orientation),
	        };
            if !new_position.is_valid() || !environment.is_cell_free(&new_position) {
                return false;
            } else {
            }
        }
        for position in self.mut_positions().iter_mut() {
	        let new_position = match direction {
		        Direction::Left => position.get_left(&environment.orientation),
		        Direction::Right => position.get_right(&environment.orientation),
	        };
            *position = new_position;
        }
        return true;
    }

}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub enum Cell {
    Free = 0,
    Blue = 1,
    Purple = 2,
    Orange = 3,
    Green = 4,
    Yellow = 5,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

#[wasm_bindgen]
pub struct BoardEnvironment {
    cells: [Cell; BOARD_LENGTH * BOARD_LENGTH],
    orientation: Orientation,
}

#[wasm_bindgen]
impl BoardEnvironment {
    pub fn new() -> BoardEnvironment {
        BoardEnvironment {
            cells: [Cell::Free; BOARD_LENGTH * BOARD_LENGTH],
            orientation: Orientation::Zero
        }
    }

    fn is_cell_free(&self, position: &Position) -> bool {
        let i = position.get_index(&self.orientation);
        return self.cells[i] == Cell::Free;
    }

    // Returns false if the cell under the given stone is free.
    fn is_blocked_below(&self, stone: &Stone) -> bool {
        for position in stone.positions().iter() {
            let position_under = position.get_under(&self.orientation);
            if !position_under.is_valid() || !self.is_cell_free(&position_under) {
                return true;
            }
        }
        return false;
    }

    // Cells with the current fixed stones on the board.
    fn get_cells(&self) -> [Cell; BOARD_LENGTH * BOARD_LENGTH] {
        return self.cells;
    }
    
    fn update_rows(&mut self) {
        for y in 0..BOARD_LENGTH {
            let mut row_complete = true;
            for x in 0..BOARD_LENGTH {
                if self.cells[y * BOARD_LENGTH + x] == Cell::Free {
                    row_complete = false;
                    continue
                }
            }
            if row_complete {
                for x in 0..BOARD_LENGTH { 
                    self.cells[y * BOARD_LENGTH + x] = Cell::Free;
                }
            }
        }
    }

    fn update_columns(&mut self) {
        for x in 0..BOARD_LENGTH {
            let mut col_complete = true;
            for y in 0..BOARD_LENGTH {
                if self.cells[y * BOARD_LENGTH + x] == Cell::Free {
                    col_complete = false;
                    continue
                }
            }
            if col_complete {
                for y in 0..BOARD_LENGTH { 
                    self.cells[y * BOARD_LENGTH + x] = Cell::Free;
                }
            }
        }
    }

    fn add(&mut self, stone: &Stone) {
         for position in stone.positions().iter() {
            let i = position.get_index(&self.orientation);    
            self.cells[i] = *stone.color();
        }
    }

    fn rotate_clockwise(&mut self) {
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

    fn rotate_counter_clockwise(&mut self) {
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

#[wasm_bindgen]
pub struct Board {
    falling_stone:  Option<Stone>,
    board_environment: BoardEnvironment,
    cells: [Cell; BOARD_LENGTH * BOARD_LENGTH],
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        Board {
            board_environment: BoardEnvironment::new(),
            falling_stone: None,
            cells: [Cell::Free; BOARD_LENGTH * BOARD_LENGTH],
        }
    }

    fn add_stone(&mut self) -> bool {
        let new_stone = Stone::new();
        for position in new_stone.positions().iter() {
            if !self.board_environment.is_cell_free(position) {
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
                if self.board_environment.is_blocked_below(stone) {
                    return false;
                }
                for position in stone.mut_positions().iter_mut() {
			        *position = position.get_under(&self.board_environment.orientation);
                }
                true
            }
        }
    }


    // Snapshot of the board with fixed stones and falling stone.
    pub fn snapshot(&mut self) -> *const Cell {
        return match &self.falling_stone {
            None => {
                self.cells = self.board_environment.get_cells();
                self.cells.as_ptr()
            }
            Some(stone) => {
                self.cells = self.board_environment.get_cells();
                for position in stone.positions().iter() {
                    let i = position.get_index(&self.board_environment.orientation);    
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
                    self.board_environment.add(stone);
                }
            }
            self.board_environment.update_rows();
            self.board_environment.update_columns();
            return self.add_stone();
        }
        return true;
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.board_environment.rotate_counter_clockwise();
    }

    pub fn rotate_clockwise(&mut self) {
        self.board_environment.rotate_clockwise();
    }

    // moveStone.
    pub fn move_stone(&mut self, direction: Direction) -> bool {
		return match &mut self.falling_stone {
            None => {
                false
            }
            Some(stone) => {
                return stone.move_unless_blocked(direction, &self.board_environment);
            }
		}
		
	}
}
