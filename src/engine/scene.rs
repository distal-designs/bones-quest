use ggez::{self, GameResult};


pub trait Scene<I, F> {
    fn update(&mut self, input: &I, flags: &mut F) -> GameResult<()>;
    fn draw(&self, flags: &F, ctx: &mut ggez::Context) -> GameResult<()>;
}
