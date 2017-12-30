use ggez::graphics;
use ggez::graphics::{Font, Text, Point, Drawable};
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


impl Scene {
    pub fn new(dialog: Vec<Command>) -> Scene {
        Scene {
            dialog,
            font:  Font::default_font().unwrap(),
            dialog_index: 0,
        }
    }
}


impl scene::Scene for Scene {
    fn update(&mut self, _: &Input, _: &mut Flags) { }

    fn draw(&self, _: &Flags, ctx: &mut ggez::Context) {
        graphics::clear(ctx);
        let (_, lines) = self.font.get_wrap(&self.dialog[self.dialog_index].text, 700);

        for (index, line) in lines.iter().enumerate() {
            Text::new(ctx, &line, &self.font)
                .unwrap()
                .draw(ctx, Point { x: 400.0, y: index as f32 * 25.0 + 100.0 }, 0.0)
                .unwrap();
        }

        graphics::present(ctx);
    }
}
