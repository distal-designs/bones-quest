use ggez;

use input::Input;
use flags::Flags;


pub trait Scene {
    fn update(&mut self, input: &Input, flags: &mut Flags);
    fn draw(&self, flags: &Flags, ctx: &mut ggez::Context);
}
