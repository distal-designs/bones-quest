use ggez::graphics;
use ggez::graphics::{Drawable, Font, Point2, Text};
use ggez;

use engine::visual_novel::command::Command;
use engine;

pub struct VisualNovel {
    dialog: Vec<Command>,
    font: Font,
    dialog_index: usize,
}

impl VisualNovel {
    pub fn new(dialog: Vec<Command>) -> Self {
        Self {
            dialog,
            font: Font::default_font().unwrap(),
            dialog_index: 0,
        }
    }
}

impl<I, F> engine::scene::Scene<I, F> for VisualNovel {
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
