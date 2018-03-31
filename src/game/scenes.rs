use ggez::{self, GameResult};

use engine::visual_novel::command::Command;
use engine::ui::Message;
use engine;

pub struct VisualNovel {
    dialog: Vec<Command>,
    dialog_index: usize,
    message: Option<Message>,
}

impl VisualNovel {
    fn apply(&mut self) {
        let commands = &mut self.dialog;
        let command = &mut commands[self.dialog_index];
        self.message = Some(Message::new(&command.text));
    }

    pub fn new(dialog: Vec<Command>) -> Self {
        let mut s = Self {
            dialog,
            dialog_index: 0,
            message: None,
        };
        s.apply();
        return s;
    }
}

impl<I, F> engine::scene::Scene<I, F> for VisualNovel {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        Ok(())
    }

    fn draw(&self, _: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        if let Some(ref message) = self.message {
            message.draw(ctx)?;
        }
        Ok(())
    }
}

pub struct Fight;

impl<I, F> engine::scene::Scene<I, F> for Fight {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}

pub struct Overworld;

impl<I, F> engine::scene::Scene<I, F> for Overworld {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}

pub struct Conquest;

impl<I, F> engine::scene::Scene<I, F> for Conquest {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}

pub struct Investigation;

impl<I, F> engine::scene::Scene<I, F> for Investigation {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}
