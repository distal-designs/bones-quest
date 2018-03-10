use ggez;

use scene::Scene;
use input::Input;
use flags::Flags;

pub trait SceneStack {
    fn pop(&mut self) -> Option<Box<Scene>>;
    fn push(&mut self, scene: Box<Scene>);
    fn update(&mut self, input: &Input, flags: &mut Flags);
    fn draw(&self, flags: &Flags, ctx: &mut ggez::Context);
}

impl SceneStack for Vec<Box<Scene>> {
    fn pop(&mut self) -> Option<Box<Scene>> {
        self.pop()
    }

    fn push(&mut self, scene: Box<Scene>) {
        self.push(scene);
    }

    fn update(&mut self, input: &Input, flags: &mut Flags) {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get_mut(current_index) {
            scene.update(input, flags);
        }
    }

    fn draw(&self, flags: &Flags, ctx: &mut ggez::Context) {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get(current_index) {
            scene.draw(flags, ctx);
        }
    }
}
