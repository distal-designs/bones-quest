use ggez::{self, GameResult};

use super::scene::Scene;


pub trait SceneStack<I, F> {
    fn pop(&mut self) -> Option<Box<Scene<I, F>>>;
    fn push(&mut self, scene: Box<Scene<I, F>>);
    fn update(&mut self, input: &I, flags: &mut F) -> GameResult<()>;
    fn draw(&self, flags: &F, ctx: &mut ggez::Context) -> GameResult<()>;
}


impl<I, F> SceneStack<I, F> for Vec<Box<Scene<I, F>>> {
    fn pop(&mut self) -> Option<Box<Scene<I, F>>> {
        self.pop()
    }


    fn push(&mut self, scene: Box<Scene<I, F>>) {
        self.push(scene);
    }


    fn update(&mut self, input: &I, flags: &mut F) -> GameResult<()> {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get_mut(current_index) {
            scene.update(input, flags)?;
        }
        Ok(())
    }


    fn draw(&self, flags: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get(current_index) {
            scene.draw(flags, ctx)?;
        }
        Ok(())
    }
}
