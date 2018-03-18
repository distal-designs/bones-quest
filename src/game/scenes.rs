use ggez::{self, GameResult};

use engine::visual_novel::command::Command;
use engine::ui::Message;
use engine;

pub struct VisualNovel {
    dialog: Vec<Command>,
    dialog_index: usize,
    message: Message,
}

impl VisualNovel {
    pub fn new(dialog: Vec<Command>) -> Self {
        let message = Message::new(&dialog[0].text);
        Self {
            dialog,
            message,
            dialog_index: 0,
        }
    }
}

impl<I, F> engine::scene::Scene<I, F> for VisualNovel {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        Ok(())
    }

    fn draw(&self, _: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        self.message.draw(ctx)?;
        Ok(())
    }
}
