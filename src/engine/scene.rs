use ggez;

pub trait Scene<I, F> {
    fn update(&mut self, input: &I, flags: &mut F);
    fn draw(&self, flags: &F, ctx: &mut ggez::Context);
}
