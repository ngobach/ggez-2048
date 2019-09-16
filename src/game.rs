use crate::common::{map_to_error, Result};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::EventsLoop;
use ggez::{event, Context, ContextBuilder};

mod board;
mod cell;
mod colors;
mod state;
mod fonts;

pub struct Game {
    ctx: Context,
    event_loop: EventsLoop,
}

impl Game {
    pub fn start(self) -> Result<()> {
        let mut ctx = self.ctx;
        let mut event_loop = self.event_loop;
        let mut game_state = state::new();
        event::run(&mut ctx, &mut event_loop, &mut game_state).map_err(map_to_error)?;
        Ok(())
    }
}

pub fn new() -> Result<Game> {
    let (u, v) = ContextBuilder::new("2048", "BachNX")
        .window_setup(WindowSetup::default().title("RUST 2048"))
        .window_mode(
            WindowMode::default()
                .resizable(false)
                .dimensions(400.0, 600.0),
        )
        .build()
        .map_err(map_to_error)?;
    Ok(Game {
        ctx: u,
        event_loop: v,
    })
}
