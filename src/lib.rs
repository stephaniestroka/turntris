mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, turntris!");
}

enum Orientation {
    Zero, Ninety, OneEighty, TwoSeventy,
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {
            x: x, y: y,
        }
    }

    fn get_index(&self, length: u8, orientation: &Orientation) -> usize {
        let l = length as usize;
        let x = self.x as usize;
        let y = self.y as usize;
        match orientation {
            Orientation::Zero => l * y + x,
            Orientation::Ninety => l * x + (l - y - 1),
            Orientation::OneEighty => l * (l - y - 1) + (l - y - 1),
            Orientation::TwoSeventy => l * (l - x - 1) + y,
        }
    }

    // Returns the position under the given position.
    // The term 'under' depends on the boards orientation: 0, 90, 180 or 270 degrees.
    // Note that the position might be outside of the board. Use `is_valid`
    // to check if it is on the board.
    fn get_under(&self, orientation: &Orientation) -> Position {
        match orientation {
            Orientation::Zero => Position::new(self.x, self.y - 1),
            Orientation::Ninety => Position::new(self.x - 1, self.y),
            Orientation::OneEighty => Position::new(self.x, self.y + 1),
            Orientation::TwoSeventy => Position::new(self.x + 1, self.y),
        }
    }

    fn is_valid(&self, length: u8) -> bool {
        return match self {
            Position{ x, y } if x >= &0 && x < &(length as i32) && y >= &0 && y < &(length as i32) => true,
            _ => false
        }
    }
}

pub struct Stone {
    consists_of: Vec<Position>,
}

impl Stone {
    // TODO: create other kinds of stones at random
    pub fn new(length: u8) -> Stone {
        let middle = (length / 2) as i32;
        return Stone {
            consists_of: vec![
                Position::new(middle, middle - 2),
                Position::new(middle, middle - 1),
                Position::new(middle, middle),
                Position::new(middle, middle + 1),
            ],
        }
    }

    fn falls(&mut self, board: &mut Board) -> bool {
        for position in self.consists_of.iter() {
            if board.is_blocked_below(position) {
                return false;
            }
        }
        let mut new_consists_of = Vec::<Position>::with_capacity(4);
        for position in self.consists_of.iter() {
            let new_position = board.cell_falls(position);
            new_consists_of.push(new_position);
        }
        self.consists_of = new_consists_of;
        return true;
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Free = 0,
    // TODO: more colors
    Blue = 1,
}

#[wasm_bindgen]
pub struct Board {
    length: u8,
    cells: Vec<Cell>,
    stones: Vec<Stone>,
    orientation: Orientation,
}

#[wasm_bindgen]
impl Board {
    pub fn new(length: u8) -> Board {
        let l = length as usize;
        Board {
            length: length,
            cells: vec![Cell::Free; l * l],
            stones: Vec::with_capacity((l * l) / 4),
            orientation: Orientation::Zero,
        }
    }

    // TODO: pass color
    fn set_cell(&mut self, position: &Position) {
        let i = position.get_index(self.length, &self.orientation);
        self.cells.insert(i, Cell::Blue);
    }

    fn free_cell(&mut self, position: &Position) {
        let i = position.get_index(self.length, &self.orientation);
        self.cells.insert(i, Cell::Free);
    }

    fn is_cell_free(&self, position: &Position) -> bool {
        let i = position.get_index(self.length, &self.orientation);
        return *self.cells.get(i).unwrap() == Cell::Free;
    }

    fn cell_falls(&mut self, position: &Position) -> Position {
        let new_position = position.get_under(&self.orientation);
        self.free_cell(&position);
        self.set_cell(&new_position);
        return new_position;
    }

    // Returns false if the cell under the given position is free.
    fn is_blocked_below(&self, position: &Position) -> bool {
        let position_under = position.get_under(&self.orientation);
        return match position.is_valid(self.length) {
            true => !self.is_cell_free(&position_under),
            false => true,
        }
    }

    pub fn cells(&self) -> *const Cell {
        let mut current_cells: Vec<Cell> = Vec::new();
        for i in 0..self.length as i32 {
            for j in 0..self.length as i32  {
                let position = Position::new(i, j);
                current_cells.push(*self.cells.get(position.get_index(self.length, &self.orientation)).unwrap());
            }
        }
        return current_cells.as_ptr();
    }

    // The length of one side of the canvas.
    pub fn length(&self) -> u8 {
        return self.length;
    }

    // Advances the game by one tick.
    pub fn tick(&mut self) {
        // States:
        // - All stones that are falling will move down.
        // - If a row is full, the row disappears and all stones will fall down.
        // - If no stone is falling, create a new stone.
    }
}
