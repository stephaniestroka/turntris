mod utils;

use std::collections::HashSet;
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

const BOARD_LENGTH: usize = 30;

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, turntris!");
}

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
            Orientation::Zero => Position::new(self.x, self.y + 1),
            Orientation::Ninety => Position::new(self.x + 1, self.y),
            Orientation::OneEighty => Position::new(self.x, self.y - 1),
            Orientation::TwoSeventy => Position::new(self.x - 1, self.y),
        }
    }

    fn get_left(&self, orientation: &Orientation) -> Position {
		match orientation {
            Orientation::Zero => Position::new(self.x - 1, self.y),
            Orientation::Ninety => Position::new(self.x, self.y - 1),
            Orientation::OneEighty => Position::new(self.x + 1, self.y),
            Orientation::TwoSeventy => Position::new(self.x, self.y + 1),
        }
	}

    fn get_right(&self, orientation: &Orientation) -> Position {
		match orientation {
            Orientation::Zero => Position::new(self.x + 1, self.y),
            Orientation::Ninety => Position::new(self.x, self.y + 1),
            Orientation::OneEighty => Position::new(self.x - 1, self.y),
            Orientation::TwoSeventy => Position::new(self.x, self.y - 1),
        }
	}

    fn is_valid(&self) -> bool {
        return match self {
            Position{ x, y } if x >= &0 && x < &(BOARD_LENGTH as i32) && y >= &0 && y < &(BOARD_LENGTH as i32) => true,
            _ => false
        }
    }
}

#[derive(Clone)]
pub struct Stone {
    consists_of: [Position; 4],
}

impl Stone {
    // TODO: create other kinds of stones at random
    pub fn new() -> Stone {
        let middle = (BOARD_LENGTH / 2) as i32;
        return Stone {
            consists_of: [
                Position::new(middle, middle - 2),
                Position::new(middle, middle - 1),
                Position::new(middle, middle),
                Position::new(middle, middle + 1),
            ],
        }
    }

    fn update(&mut self, consists_of: [Position; 4]) {
        self.consists_of = consists_of;
    }
    
    fn positions(&self) -> &[Position; 4] {
        return &self.consists_of;
    }

    fn lowest_positions(&self, orientation: &Orientation) -> HashSet<Position> {
        let mut set = HashSet::new();
        for (i, pos1) in self.consists_of.iter().enumerate() {
            let mut min_vertical = match orientation {
                Orientation::Zero | Orientation::OneEighty => pos1.y,
                Orientation::Ninety | Orientation::TwoSeventy => pos1.x
            };
            for j in (i+1)..self.consists_of.len() {
                let pos2 = self.consists_of.get(j).unwrap();
                match orientation {
                    Orientation::Zero => {
                        if pos1.x == pos2.x && pos2.y > pos1.y {
                            min_vertical = pos2.y;
                        }
                    },
                    Orientation::Ninety => {
                        if pos1.y == pos2.y && pos2.x > pos1.x {
                            min_vertical = pos2.x;
                        }
                    },
                    Orientation::OneEighty => {
                        if pos1.x == pos2.x && pos2.y < pos1.y {
                            min_vertical = pos2.y;
                        }
                    },
                    Orientation::TwoSeventy => {
                        if pos1.y == pos2.y && pos2.x < pos1.x {
                            min_vertical = pos2.x;
                        }
                    }
                };
            }
            match orientation {
                Orientation::Zero | Orientation::OneEighty => set.insert(Position::new(pos1.x, min_vertical)),
                Orientation::Ninety | Orientation::TwoSeventy => set.insert(Position::new(min_vertical, pos1.y))
            };
        }
        return set;
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
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

#[wasm_bindgen]
pub struct Board {
    cells: [Cell; BOARD_LENGTH * BOARD_LENGTH],
    stones: Vec<Stone>,
    orientation: Orientation,
}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        Board {
            cells: [Cell::Free; BOARD_LENGTH * BOARD_LENGTH],
            stones: Vec::with_capacity((BOARD_LENGTH * BOARD_LENGTH ) / 4),
            orientation: Orientation::Zero,
        }
    }

    // TODO: pass color
	// Returns false if the cell is already set.
    fn set_cell(&mut self, position: &Position) -> bool {
        let i = position.get_index(&self.orientation);
        match self.cells[i] {
            Cell::Free => {
				self.cells[i] = Cell::Blue;
				true
			},
            _ => false 
        }
    }

    fn add_stone(&mut self) -> bool {
        let stone = Stone::new();
        for position in stone.positions().iter() {
            if !self.set_cell(position) {
                return false;
            }
        }
        self.stones.push(stone);
        return true;
    }

    fn free_cell(&mut self, position: &Position) {
        let i = position.get_index(&self.orientation);
        self.cells[i] = Cell::Free;
    }

    fn is_cell_free(&self, position: &Position) -> bool {
        let i = position.get_index(&self.orientation);
        return self.cells[i] == Cell::Free;
    }

    // Returns false if the cell under the given position is free.
    fn is_blocked_below(&self, position: &Position) -> bool {
        let position_under = position.get_under(&self.orientation);
        return match position_under.is_valid() {
            true => !self.is_cell_free(&position_under),
            false => true,
        }
    }

    fn stones_fall(&mut self) -> bool {
        let mut idxs = Vec::<usize>::with_capacity(self.stones.len());
        {
            for (i, stone) in self.stones.iter().enumerate() {
                let mut is_blocked = false;
                for position in stone.lowest_positions(&self.orientation).iter() {
                    if self.is_blocked_below(position) {
                        is_blocked = true;
                    }
                }
                if !is_blocked {
                    idxs.push(i);
                }
            }
        }

        if idxs.is_empty() {
            return false;
        }
        {
            for i in idxs.iter() {
                let mut new_positions: [Position; 4] = [Position::new(0, 0); 4];
                {
                    let stone = self.stones.get_mut(*i).unwrap();
                    let positions = stone.positions().clone();
                    for (index, position) in positions.iter().enumerate() {
                        self.free_cell(position);
						let new_position = position.get_under(&self.orientation);
                        new_positions[index] = new_position;
                    }
                    for position in new_positions.iter() {
                        self.set_cell(position);
					}
                }
                {
                    let stone = self.stones.get_mut(*i).unwrap();
                    stone.update(new_positions);
                }
            }
        }
        return true;
    }

    pub fn cells(&self) -> *const Cell {
        return self.cells.as_ptr();
        /*let mut current_cells: Vec<Cell> = Vec::new();
        for j in 0..BOARD_LENGTH as i32 {
            for i in 0..BOARD_LENGTH as i32  {
                let position = Position::new(i, j);
                current_cells.push(self.cells[position.get_index(&self.orientation)]);
            }
        }
        return current_cells.as_ptr();*/
    }

    // The length of one side of the canvas.
    pub fn length(&self) -> usize {
        return BOARD_LENGTH;
    }

    // Advances the game by one tick.
    pub fn tick(&mut self) -> bool {
        // States:
        // - All stones that are falling will move down.
        // - If a row is full, the row disappears and all stones will fall down.
        // - If no stone is falling, create a new stone.
        if !self.stones_fall() {
            return self.add_stone();
        }
        return true;
    }

    // moveStone.
    pub fn move_stone(&mut self, direction: Direction) -> bool {
		if self.stones.len() < 1 {
			return false;
		}
		let current_stone = self.stones.get(self.stones.len() - 1);
		log("positions of current stone:");
        let mut new_positions: [Position; 4] = [Position::new(0, 0); 4];
		let positions = current_stone.unwrap().positions().clone();
		for (index, position) in positions.iter().enumerate() {
			self.free_cell(position);
			let new_position = match direction{
				Direction::Left => position.get_left(&self.orientation),
				Direction::Right => position.get_right(&self.orientation),
			};
			new_positions[index] = new_position;
		}
		for position in new_positions.iter() {
			self.set_cell(position);
		}
		{
			let stone = self.stones.get_mut(0).unwrap();
			stone.update(new_positions);
		}
		return true;
	}
}
