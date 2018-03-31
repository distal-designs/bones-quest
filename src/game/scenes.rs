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
    pub fn new(dialog: Vec<Command>) -> Self {
        Self {
            dialog,
            dialog_index: 0,
            message: None,
        }
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
