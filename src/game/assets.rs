use super::fonts;
use crate::game::fonts::FontList;
use ggez::{Context, GameResult};

pub struct Assets {
    fonts: fonts::FontList,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            fonts: fonts::load_font(ctx)?,
        })
    }

    pub fn fonts(&self) -> &FontList {
        &self.fonts
    }
}
