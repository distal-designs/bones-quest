use ggez;

use scene::Scene;
use input::Input;
use flags::Flags;

pub trait SceneStack {
    fn pop(&mut self) -> Option<Box<Scene>>;
    fn push (&mut self, scene: Box<Scene>);
    fn update(&mut self, input: &Input, flags: &mut Flags);
    fn draw(&self, flags: &Flags, ctx: &mut ggez::Context);
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
