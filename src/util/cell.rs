use wasm_bindgen::prelude::*;

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