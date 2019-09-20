use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, Rect};
use mint::Point2;
use stretch::geometry::Size;
use stretch::node::{Node, Stretch};
use stretch::style::{Dimension, FlexWrap, Style};

use super::brush::rounded_rect;
use super::cell::Cell;
use super::layout::{RectChain, to_rect};

pub struct Board {
    matrix: [[Cell; 4]; 4],
    root: Rect,
    cells: [[Rect; 4]; 4],
}

impl Board {
    pub fn draw(&self, ctx: &mut Context, dst: Point2<f32>) -> GameResult<()> {
        for i in 0..4 {
            for j in 0..4 {
                let mut rect = rounded_rect(ctx, self.cells[i][j], 8., super::colors::CELL_COLORS[(i * 4 + j) % 12])?;
                ggez::graphics::draw(ctx, &rect, DrawParam::default().dest(dst));
            }
        }
        Ok(())
    }
}

pub fn new(r: Rect) -> Board {
    let mut s = Stretch::new();
    let vcells: Vec<Node> = (0..16)
        .map(|_| {
            let child = s
                .new_node(
                    Style {
                        size: Size {
                            width: Dimension::Percent(1.),
                            height: Dimension::Percent(1.),
                        },
                        ..Default::default()
                    },
                    vec![],
                )
                .unwrap();
            s.new_node(
                Style {
                    size: Size {
                        width: Dimension::Percent(0.25),
                        height: Dimension::Percent(0.25),
                    },
                    padding: stretch::geometry::Rect {
                        top: Dimension::Points(8.0),
                        end: Dimension::Points(8.0),
                        bottom: Dimension::Points(8.0),
                        start: Dimension::Points(8.0),
                    },
                    ..Default::default()
                },
                vec![child],
            )
                .unwrap()
        })
        .collect();

    let mut cells: [[Rect; 4]; 4] = Default::default();
    let mut root = s
        .new_node(
            Style {
                size: Size {
                    width: Dimension::Points(r.w),
                    height: Dimension::Points(r.h),
                },
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            vcells.clone(),
        )
        .unwrap();
    s.compute_layout(root, Size::undefined()).unwrap();

    let root = to_rect(s.layout(root).unwrap());
    for i in 0..4 {
        for j in 0..4 {
            let inner = to_rect(s.layout(s.children(vcells[i * 4 + j]).unwrap()[0]).unwrap());
            let outer = to_rect(s.layout(vcells[i * 4 + j]).unwrap());
            cells[i][j] = RectChain::from(inner).chain(outer).into();
        }
    }
    Board {
        matrix: [[Cell::None; 4]; 4],
        root: root.into(),
        cells,
    }
}
