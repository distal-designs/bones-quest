use ggez::graphics::Font;
use ggez;

use visual_novel::command::Command;
use scene;
use input::Input;
use flags::Flags;

pub struct Scene {
    dialog: Vec<Command>,
    font: Font,
    dialog_index: usize,
}

impl scene::Scene for Scene {
    fn update(&mut self, input: &Input, flags: &mut Flags) { unimplemented!() }
    fn draw(&self, flags: &Flags, ctx: &mut ggez::Context) { unimplemented!() }
}
