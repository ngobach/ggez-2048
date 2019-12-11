use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{Context, GameResult};

pub fn rounded_rect(ctx: &mut Context, rect: Rect, radius: f32, color: Color) -> GameResult<Mesh> {
    let mut mb = MeshBuilder::new();
    let fill = DrawMode::fill();
    mb.rectangle(
        fill,
        Rect::new(rect.x + radius, rect.y, rect.w - radius * 2., rect.h),
        color,
    );
    mb.rectangle(
        fill,
        Rect::new(rect.x, rect.y + radius, rect.w, rect.h - radius * 2.),
        color,
    );
    for x in [rect.x + radius, rect.x + rect.w - radius].into_iter() {
        for y in [rect.y + radius, rect.y + rect.h - radius].into_iter() {
            mb.circle(fill, [*x, *y], radius, 0.05, color);
        }
    }
    mb.build(ctx)
}
