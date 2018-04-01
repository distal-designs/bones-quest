use ggez::graphics::{get_color, set_color, Color};
use ggez::{Context, GameResult};

use regex::Regex;

pub fn with_color<F, T>(ctx: &mut Context, color: &Color, fun: F) -> GameResult<T>
where
    F: FnOnce(&mut Context) -> GameResult<T>,
{
    let current_color = get_color(ctx);
    set_color(ctx, *color)?;
    let a = fun(ctx);
    set_color(ctx, current_color)?;
    a
}

pub fn from_hex(hex: &str) -> Color {
    static INVALID_HEX_MESSAGE: &'static str = "Hex color specified is invalid!";
    let re = Regex::new(r"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$").unwrap();
    let captures = re.captures(hex).expect(INVALID_HEX_MESSAGE);
    let capture = captures.get(1).expect(INVALID_HEX_MESSAGE);
    match capture.as_str() {
        x if x.len() == 6 => Color::from_rgb_u32(x.parse().expect(INVALID_HEX_MESSAGE)),
        x if x.len() == 8 => Color::from_rgba_u32(x.parse().expect(INVALID_HEX_MESSAGE)),
        _ => panic!(INVALID_HEX_MESSAGE),
    }
}
