use ggez::graphics;
use ggez::graphics::{Drawable, Point2, Text};
use ggez::{self, GameResult};

use engine::visual_novel::command::Command;
use engine;

pub struct VisualNovel {
    dialog: Vec<Command>,
    dialog_index: usize,
}

impl VisualNovel {
    pub fn new(dialog: Vec<Command>) -> Self {
        Self {
            dialog,
            dialog_index: 0,
        }
    }
}

impl<I, F> engine::scene::Scene<I, F> for VisualNovel {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        Ok(())
    }

    fn draw(&self, _: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        graphics::clear(ctx);
        let font = ctx.default_font.clone();

        ctx.default_font
            .get_wrap(&self.dialog[self.dialog_index].text, 700)
            .1
            .iter()
            .enumerate()
            .for_each(|(index, line)| {
                Text::new(ctx, &line, &font)
                    .unwrap()
                    .draw(ctx, Point2::new(400.0, index as f32 * 25.0 + 100.0), 0.0)
                    .unwrap();
            });

        graphics::present(ctx);
        Ok(())
    }
}
