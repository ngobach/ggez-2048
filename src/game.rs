use crate::common::{Result};

pub struct Game {
}

impl Game {
    pub fn start(&mut self) -> Result<()> {
        eprintln!("Game is starting!");
        Ok(())
    }
}

pub fn new() -> Game {
    Game {
    }
}