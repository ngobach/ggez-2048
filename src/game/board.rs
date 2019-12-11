use ggez::graphics::{self, Align, DrawMode, DrawParam, MeshBuilder, Rect, Scale};
use ggez::{Context, GameResult};
use mint::Point2;
use stretch::geometry::Size;
use stretch::node::{Node, Stretch};
use stretch::style::{Dimension, FlexWrap, Style};

use super::brush::rounded_rect;
use super::cell::Cell;
use super::layout::{to_rect, RectChain};

pub struct Board {
    matrix: [[Cell; 4]; 4],
    root: Rect,
    cells: [[Rect; 4]; 4],
}

fn pick_one<T: Clone>(items: Vec<T>) -> Option<T> {
    let count = items.len();
    if count == 0 {
        return None;
    }
    let it = rand::random::<usize>() % count;
    Some(items[it].clone())
}

fn dump_board(b: &[[Cell; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            match b[i][j] {
                Cell::Some {value} => print!("{} ", value),
                Cell::None => print!("0 "),
            }
        }
        println!()
    }
    println!()
}

impl Board {
    pub fn draw(
        &self,
        ctx: &mut Context,
        dst: Point2<f32>,
        asset: &super::assets::Assets,
    ) -> GameResult<()> {
        for i in 0..4 {
            for j in 0..4 {
                match self.matrix[i][j] {
                    Cell::None => {
                        let rect = rounded_rect(ctx, self.cells[i][j], 8., *super::colors::NORD_1)?;
                        ggez::graphics::draw(ctx, &rect, DrawParam::default().dest(dst))?;
                    }
                    Cell::Some { value } => {
                        let rect = rounded_rect(
                            ctx,
                            self.cells[i][j],
                            8.,
                            super::colors::CELL_COLORS[(value % 12) as usize],
                        )?;
                        let mut label = ggez::graphics::Text::new(format!("{}", 1 << value));
                        label.set_font(asset.fonts().normal(), Scale::uniform(24.));
                        let cell = &self.cells[i][j];
                        label.set_bounds([cell.w, cell.h], Align::Center);
                        ggez::graphics::draw(ctx, &rect, DrawParam::default().dest(dst))?;
                        ggez::graphics::draw(
                            ctx,
                            &label,
                            DrawParam::default().dest([
                                dst.x + cell.x,
                                dst.y + cell.y + 28., // Magic Number :")
                            ]),
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn next(&mut self) -> bool {
        let mut candidates: Vec<(u8, u8)> = vec![];
        for i in 0..4 {
            for j in 0..4 {
                if let Cell::None = self.matrix[i][j] {
                    candidates.push((i as u8, j as u8));
                }
            }
        }
        let picked = pick_one(candidates);
        if let Some(p) = picked {
            self.matrix[p.0 as usize][p.1 as usize] = Cell::Some { value: 1 };
            true
        } else {
            false
        }
    }

    pub fn send_vec(&mut self, v: (i8, i8)) -> u32 {
        let mut bonus: u32 = 0;
        fn safe_num(x: i8) -> bool {
            x >= 0 && x <= 10
        }
        fn rotate_vec(v: (i8, i8)) -> (i8, i8) {
            (-v.1, v.0)
        }
        fn rotate_mat(m: [[Cell; 4]; 4]) -> [[Cell; 4]; 4] {
            let mut dummy = [[Cell::None; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    dummy[i][j] = m[3-j][i]
                }
            }
            dummy
        }
        let mut vec = v.clone();
        let mut mat = self.matrix.clone();
        // Rotate until vector point to left
        while vec.0 != -1 {
            mat = rotate_mat(mat);
            vec = rotate_vec(vec);
        }
        for i in 0..4 {
            for j in 0..3 {
                if let (Cell::Some { value: n1 }, Cell::Some { value: n2 }) = (mat[i][j], mat[i][j+1]) {
                    if n1 == n2 {
                        mat[i][j] = Cell::Some { value: n1 + 1};
                        mat[i][j+1] = Cell::None;
                        bonus += 1 << (n1 + 1);
                    }
                }
            }
        }
        for turn in 0..4 {
            for i in 0..4 {
                for j in 1..4 {
                    if let (Cell::None, Cell::Some { value: _ }) = (mat[i][j-1], mat[i][j]) {
                        mat[i][j-1] = mat[i][j];
                        mat[i][j] = Cell::None;
                    }
                }
            }
        }
        while v != vec {
            vec = rotate_vec(vec);
            mat = rotate_mat(mat);
        }
        self.matrix = mat;
        bonus
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
    let root = s
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
