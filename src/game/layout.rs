use ggez::graphics::Rect;
use stretch::geometry::Size;
use stretch::node::{Node, Stretch};
use stretch::number::Number;
use stretch::result::Layout;
use stretch::style::{AlignItems, AlignSelf, Dimension, FlexDirection, JustifyContent, Style};

pub struct GameLayout {
    root: Stretch,
    title: Node,
    score: Node,
    score_text: Node,
    board: Node,
}

pub fn to_rect(layout: &Layout) -> Rect {
    Rect {
        x: layout.location.x,
        y: layout.location.y,
        w: layout.size.width,
        h: layout.size.height,
    }
}

pub struct RectChain(Rect);

impl RectChain {
    pub fn chain(mut self, r: Rect) -> Self {
        self.0.x += r.x;
        self.0.y += r.y;
        self
    }

    pub fn from(r: Rect) -> RectChain {
        RectChain(r)
    }

    pub fn into(self) -> Rect {
        self.0
    }
}

impl GameLayout {
    pub fn title(&self) -> Rect {
        to_rect(self.root.layout(self.title).unwrap())
    }
    pub fn score(&self) -> Rect {
        to_rect(self.root.layout(self.score).unwrap())
    }
    pub fn score_text(&self) -> Rect {
        RectChain::from(to_rect(self.root.layout(self.score_text).unwrap()))
            .chain(self.score())
            .into()
    }
    pub fn board(&self) -> Rect {
        to_rect(self.root.layout(self.board).unwrap())
    }
}

pub fn compute_layout() -> GameLayout {
    let mut root = Stretch::new();
    let title = root
        .new_node(
            Style {
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Points(50.0),
                },
                align_self: AlignSelf::Center,
                flex_grow: 1.0,
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let score_text = root
        .new_node(
            Style {
                size: Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Points(24.),
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let score = root
        .new_node(
            Style {
                flex_grow: 1.0,
                size: Size {
                    width: Dimension::Auto,
                    height: Dimension::Auto,
                },
                padding: stretch::geometry::Rect {
                    top: Dimension::Points(4.0),
                    bottom: Dimension::Points(4.0),
                    start: Dimension::Points(4.0),
                    end: Dimension::Points(4.0),
                },
                margin: stretch::geometry::Rect {
                    top: Dimension::Points(4.0),
                    bottom: Dimension::Points(4.0),
                    start: Dimension::Points(8.0),
                    end: Dimension::Points(8.0),
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            vec![score_text],
        )
        .unwrap();
    let board = root
        .new_node(
            Style {
                aspect_ratio: Number::Defined(1.0),
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let head = root
        .new_node(
            Style {
                flex_grow: 1.0,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            vec![title, score],
        )
        .unwrap();
    let screen = root
        .new_node(
            Style {
                size: Size {
                    width: Dimension::Percent(1.0),
                    height: Dimension::Percent(1.0),
                },
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            vec![head, board],
        )
        .unwrap();
    root.compute_layout(
        screen,
        Size {
            width: Number::Defined(crate::consts::WINDOW_SIZE[0]),
            height: Number::Defined(crate::consts::WINDOW_SIZE[1]),
        },
    )
    .unwrap();

    GameLayout {
        root,
        title,
        score,
        score_text,
        board,
    }
}
