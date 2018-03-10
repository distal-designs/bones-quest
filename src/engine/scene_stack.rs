use ggez;

use scene::Scene;

pub trait SceneStack<I, F> {
    fn pop(&mut self) -> Option<Box<Scene<I, F>>>;
    fn push(&mut self, scene: Box<Scene<I, F>>);
    fn update(&mut self, input: &I, flags: &mut F);
    fn draw(&self, flags: &F, ctx: &mut ggez::Context);
}

impl<I, F> SceneStack<I, F> for Vec<Box<Scene<I, F>>> {
    fn pop(&mut self) -> Option<Box<Scene<I, F>>> {
        self.pop()
    }

    fn push(&mut self, scene: Box<Scene<I, F>>) {
        self.push(scene);
    }

    fn update(&mut self, input: &I, flags: &mut F) {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get_mut(current_index) {
            scene.update(input, flags);
        }
    }

    fn draw(&self, flags: &F, ctx: &mut ggez::Context) {
        let current_index = self.len() - 1;
        if let Some(scene) = self.get(current_index) {
            scene.draw(flags, ctx);
        }
    }
}
