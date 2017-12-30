use ggez::graphics::{Font};

use visual_novel::command::Command;

pub struct Scene {
    dialog: Vec<Command>,
    font: Font,
    dialog_index: usize,
}
