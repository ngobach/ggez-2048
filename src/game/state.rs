use ggez::event::EventHandler;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, Context, GameError, GameResult};

use super::{
    assets, board,
    layout::{compute_layout, GameLayout},
};
use ggez::graphics::{Align, DrawMode, DrawParam, MeshBuilder, Scale};

pub struct GameState {
    board: board::Board,
    assets: assets::Assets,
    layout: GameLayout,

    score: u32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let layout = compute_layout();
        let mut board = board::new(layout.board());
        board.next();
        Ok(Self {
            board,
            assets: assets::Assets::load(ctx)?,
            layout,
            score: 0,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        let mut vector: Option<(i8, i8)> = match keycode {
            KeyCode::Left => Some((-1, 0)),
            KeyCode::Right => Some((1, 0)),
            KeyCode::Up => Some((0, -1)),
            KeyCode::Down => Some((0, 1)),
            _ => None,
        };
        if let Some(v) = vector {
            let bonus = self.board.send_vec(v);
            self.board.next();
            self.score += bonus;
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, *super::colors::BACKGROUND);
        let mut text = graphics::Text::new("2048");
        text.set_font(self.assets.fonts().bold(), Scale::uniform(48.0));
        text.set_bounds(
            [self.layout.title().w, self.layout.title().h],
            Align::Center,
        );
        graphics::draw(
            ctx,
            &text,
            ([self.layout.title().x, self.layout.title().y],),
        )?;
        let mut score_bg = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                self.layout.score(),
                *super::colors::SCORE_BG,
            )
            .build(ctx)?;
        graphics::draw(ctx, &score_bg, DrawParam::default())?;
        let mut score_text = graphics::Text::new(format!("Score: {}", self.score));
        score_text.set_bounds(
            [self.layout.score_text().w, self.layout.score_text().h],
            Align::Center,
        );
        score_text.set_font(self.assets.fonts().normal(), Scale::uniform(24.));
        graphics::draw(
            ctx,
            &score_text,
            DrawParam::default().dest([self.layout.score_text().x, self.layout.score_text().y]),
        )?;
        self.board
            .draw(ctx, self.layout.board().point(), &self.assets)?;
        graphics::present(ctx)?;
        Ok(())
    }
}
