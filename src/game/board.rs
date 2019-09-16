use super::cell::Cell;

pub struct Board {
    matrix: [[Cell; 4]; 4],
}

impl Board {}

pub fn new() -> Board {
    Board {
        matrix: [[Cell::None; 4]; 4],
    }
}
