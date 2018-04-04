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
    let re = Regex::new(r"^#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})$").unwrap();
    let captures = re.captures(hex).unwrap();
    let capture = captures.get(1).unwrap();
    match capture.as_str() {
        x if x.len() == 6 => Color::from_rgb_u32(u32::from_str_radix(x, 16).unwrap()),
        x if x.len() == 8 => Color::from_rgba_u32(u32::from_str_radix(x, 16).unwrap()),
        _ => panic!(),
    }
}
