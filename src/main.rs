mod common;
mod consts;
mod game;

fn main() -> common::Result<()> {
    let g = game::new()?;
    g.start()?;
    Ok(())
}
