use super::cell::Cell;
use ggez::graphics::Rect;
use ggez::GameResult;
use std::task::Context;

use mint::Point2;
use stretch::node::{Node, Stretch};
use stretch::style::Style;

pub struct Board {
    matrix: [[Cell; 4]; 4],
    stretch: Stretch,
    root: Node,
    cells: [[Node; 4]; 4],
}

impl Board {
    pub fn draw(ctx: &mut Context, dst: Point2<f32>) -> GameResult<()> {
        Ok(())
    }
}

pub fn new(r: Rect) -> Board {
    let mut s = Stretch::new();
    let vcells: Vec<Node> = (0..16)
        .map(|_| s.new_node(Default::default(), vec![]).unwrap())
        .collect();
    let mut iter = vcells.iter();
    let cells: [[Node; 4]; 4] = [
        [
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
        ],
        [
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
        ],
        [
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
        ],
        [
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
            *iter.next().unwrap(),
        ],
    ];
    let mut root = s
        .new_node(
            Style {
                ..Default::default()
            },
            vcells,
        )
        .unwrap();
    Board {
        matrix: [[Cell::None; 4]; 4],
        stretch: s,
        root,
        cells,
    }
}
