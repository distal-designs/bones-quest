use ggez::graphics::Color;
use regex::Regex;

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
