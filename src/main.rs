mod common;
mod game;

fn main() -> common::Result<()> {
    let g = game::new()?;
    g.start()?;
    Ok(())
}
