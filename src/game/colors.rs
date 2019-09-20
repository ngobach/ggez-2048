use ggez::graphics::Color;


lazy_static! {
    pub static ref NORD_0: Color = Color::from_rgb_u32(0x2E3440);
    pub static ref NORD_1: Color = Color::from_rgb_u32(0x3B4252);
    pub static ref NORD_2: Color = Color::from_rgb_u32(0x434C5E);
    pub static ref NORD_3: Color = Color::from_rgb_u32(0x4C566A);

    pub static ref NORD_4: Color = Color::from_rgb_u32(0xD8DEE9);
    pub static ref NORD_5: Color = Color::from_rgb_u32(0xE5E9F0);
    pub static ref NORD_6: Color = Color::from_rgb_u32(0xECEFF4);

    pub static ref BACKGROUND: Color = *NORD_2;
    pub static ref TITLE: Color = *NORD_0;
    pub static ref SCORE_BG: Color = *NORD_3;
    pub static ref CELL_COLORS: [Color; 12] = [
        Color::from_rgb_u32(0xEC9A29),
        Color::from_rgb_u32(0xEDA33C),
        Color::from_rgb_u32(0xEFAC4F),
        Color::from_rgb_u32(0xF1B563),

        Color::from_rgb_u32(0xEE4266),
        Color::from_rgb_u32(0xEF5373),
        Color::from_rgb_u32(0xF16481),
        Color::from_rgb_u32(0xF2758F),

        Color::from_rgb_u32(0x3AA0A1),
        Color::from_rgb_u32(0x50AAAC),
        Color::from_rgb_u32(0x66B5B6),
        Color::from_rgb_u32(0x7CBFC0),
    ];
}
