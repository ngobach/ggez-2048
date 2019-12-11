use crate::common::{map_to_error, Result};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::EventsLoop;
use ggez::{event, Context, ContextBuilder};
use std::path;

mod assets;
mod board;
mod brush;
mod cell;
mod colors;
mod fonts;
mod layout;
mod state;

pub struct Game {
    ctx: Context,
    event_loop: EventsLoop,
}

impl Game {
    pub fn start(self) -> Result<()> {
        let mut ctx = self.ctx;
        let mut event_loop = self.event_loop;
        let mut game_state = state::GameState::new(&mut ctx).map_err(map_to_error)?;
        event::run(&mut ctx, &mut event_loop, &mut game_state).map_err(map_to_error)?;
        Ok(())
    }
}

pub fn new() -> Result<Game> {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (u, v) = ContextBuilder::new("2048", "BachNX")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("GGEZ 2048"))
        .window_mode(
            WindowMode::default()
                .resizable(false)
                .dimensions(super::consts::WINDOW_SIZE[0], super::consts::WINDOW_SIZE[1]),
        )
        .build()
        .map_err(map_to_error)?;
    Ok(Game {
        ctx: u,
        event_loop: v,
    })
}
