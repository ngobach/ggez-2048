use ggez::event::EventHandler;
use ggez::{graphics, Context, GameError};
use mint;

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
        let mut text = graphics::Text::new("GGEZ 2048");
        graphics::draw(ctx, &text, ([2.0,2.0],));
        graphics::present(ctx)?;
        Ok(())
    }
}
