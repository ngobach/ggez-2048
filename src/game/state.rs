use ggez::event::EventHandler;
use ggez::{Context, GameError, graphics};

use super::board;

pub struct GameState {
    board: board::Board,
}

impl GameState {
    fn new() -> Self {
        Self {
            board: board::new(),
        }
    }
}

pub fn new() -> GameState {
    GameState::new()
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, super::colors::BACKGROUND);
        graphics::present(ctx)?;
        Ok(())
    }
}