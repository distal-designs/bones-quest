use scene::Scene;
use ggez;

pub trait SceneStack<W> {
    fn pop(&mut self) -> Option<Box<Scene<W>>>;
    fn push (&mut self, scene: Box<Scene<W>>);
    fn update(&mut self, world: &mut W);
    fn draw(&self, world: &W, ctx: &mut ggez::Context);
}
