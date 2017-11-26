use scene::Scene;
use ggez;

pub trait SceneStack<W> {
    fn pop(&mut self) -> Option<Box<Scene<W>>>;
    fn push (&mut self, scene: Box<Scene<W>>);
    fn update(&mut self, world: &mut W);
    fn draw(&self, world: &W, ctx: &mut ggez::Context);
}

impl<W> SceneStack<W> for Vec<Box<Scene<W>>> {
    fn pop(&mut self) -> Option<Box<Scene<W>>> {
        self.pop()
    }

    fn push (&mut self, scene: Box<Scene<W>>) {
        self.push(scene);
    }

    fn update(&mut self, world: &mut W) {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get_mut(current_index) {
            scene.update(world);
        }
    }

    fn draw(&self, world: &W, ctx: &mut ggez::Context) {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get(current_index) {
            scene.draw(world, ctx);
        }
    }
}
