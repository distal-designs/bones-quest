use ggez;

pub trait Scene<State> {
    fn update(&mut self, state: &mut State);
    fn draw(&self, state: &State, ctx: &mut ggez::Context);
}
