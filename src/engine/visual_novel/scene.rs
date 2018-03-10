use ggez::graphics;
use ggez::graphics::{Drawable, Font, Point2, Text};
use ggez;

use super::command::Command;
use super::super::scene;

pub struct Scene {
    dialog: Vec<Command>,
    font: Font,
    dialog_index: usize,
}

impl Scene {
    pub fn new(dialog: Vec<Command>) -> Scene {
        Scene {
            dialog,
            font: Font::default_font().unwrap(),
            dialog_index: 0,
        }
    }
}

impl<I, F> scene::Scene<I, F> for Scene {
    fn update(&mut self, _: &I, _: &mut F) {}

    fn draw(&self, _: &F, ctx: &mut ggez::Context) {
        graphics::clear(ctx);

        self.font
            .get_wrap(&self.dialog[self.dialog_index].text, 700)
            .1
            .iter()
            .enumerate()
            .for_each(|(index, line)| {
                Text::new(ctx, &line, &self.font)
                    .unwrap()
                    .draw(ctx, Point2::new(400.0, index as f32 * 25.0 + 100.0), 0.0)
                    .unwrap();
            });

        graphics::present(ctx);
    }
}
