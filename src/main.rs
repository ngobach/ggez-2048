mod game;
mod common;

fn main() {
    let mut g = game::new();
    g.start().unwrap();
}