use ggez::graphics::Font;
use ggez::GameResult;
use std::path::PathBuf;

const FONT_NORMAL: &str = "/assets/Inter-Medium.ttf";
const FONT_BOLD: &str = "/assets/Inter-Bold.ttf";

pub struct FontList {
    normal: Font,
    bold: Font,
}

impl FontList {
    pub fn normal(&self) -> Font {
        self.normal.clone()
    }

    pub fn bold(&self) -> Font {
        self.bold.clone()
    }
}

pub fn load_font(ctx: &mut ggez::Context) -> GameResult<FontList> {
    Ok(FontList {
        normal: Font::new(ctx, PathBuf::from(FONT_NORMAL))?,
        bold: Font::new(ctx, PathBuf::from(FONT_BOLD))?,
    })
}
