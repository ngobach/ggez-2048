mod game;
mod common;

fn main() -> common::Result<()> {
    let g = game::new()?;
    g.start()?;
    Ok(())
}