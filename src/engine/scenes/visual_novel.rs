use std::collections::HashMap;
use std::mem;

use ggez::event::Keycode::{Left, Right};
use ggez::graphics::{DrawParam, Drawable, Image, Point2};
use ggez::{self, GameResult};

use engine::draw_cache::DrawCache;
use engine::input::Input;
use engine::ui::{Background, BackgroundCache, Character, Dialog, DialogCache, Portrait};
use engine::visual_novel::command::{BackgroundCommand, Command, PortraitCommand};
use engine::{self, color};

pub struct VisualNovel {
    commands: Vec<Command>,
    command_index: usize,
    dialog: Option<DrawCache<Dialog, DialogCache>>,
    background: Option<DrawCache<Background, BackgroundCache>>,
    status: Status,
    characters: HashMap<String, DrawCache<Character, Image>>,
}

impl VisualNovel {
    fn apply(&mut self) {
        let commands = &mut self.commands;
        let command = &mut commands[self.command_index];

        VisualNovel::apply_characters(&mut self.characters, command);
        VisualNovel::apply_dialog(&mut self.dialog, command);
        VisualNovel::apply_background(&mut self.background, command);
    }

    fn apply_characters(
        characters: &mut HashMap<String, DrawCache<Character, Image>>,
        command: &Command,
    ) {
        if let Some(ref positions) = command.positions {
            for (name, position) in positions {
                characters
                    .entry(name.clone())
                    .or_insert(DrawCache::new(Character {
                        name: name.clone(),
                        direction: position.direction.clone(),
                        position: position.position.clone(),
                    }));
            }
        }
    }

    fn apply_background(
        background: &mut Option<DrawCache<Background, BackgroundCache>>,
        command: &Command,
    ) {
        match &command.background {
            Some(BackgroundCommand::Hide) => {
                mem::replace(background, None);
            }
            Some(BackgroundCommand::Color(hex)) => {
                let new_bg = DrawCache::new(Background::Color(color::from_hex(&hex)));
                mem::replace(background, Some(new_bg));
            }
            Some(BackgroundCommand::Image(_)) => unimplemented!(),
            None => {}
        };
    }

    fn apply_dialog(dialog: &mut Option<DrawCache<Dialog, DialogCache>>, command: &Command) {
        let portrait = match (&dialog, &command.portrait) {
            (_, Some(PortraitCommand::Show(character, style))) => Some(Portrait {
                character: character.clone(),
                style: style.clone(),
            }),
            (Some(dialog_draw_cache), None) => dialog_draw_cache.as_ref().portrait.clone(),
            (_, Some(PortraitCommand::Hide)) => None,
            (None, None) => None,
        };
        let text = command.text.clone();
        mem::replace(dialog, Some(DrawCache::new(Dialog { text, portrait })));
    }

    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            commands,
            command_index: 0,
            status: Status::PendingCommands,
            dialog: None,
            background: None,
            characters: HashMap::new(),
        }
    }
}

impl<F> engine::scene::Scene<Input, F> for VisualNovel {
    fn update(&mut self, input: &Input, _: &mut F) -> GameResult<()> {
        for keycode in input.pressed() {
            self.command_index = match (keycode, self.command_index) {
                (Left, x) if x != 0 => {
                    self.status = Status::PendingCommands;
                    x - 1
                }
                (Right, x) if x != self.commands.len() - 1 => {
                    self.status = Status::PendingCommands;
                    x + 1
                }
                (_, x) => x,
            }
        }
        if let Status::PendingCommands = self.status {
            self.apply();
            self.status = Status::CommandsApplied;
        }
        Ok(())
    }

    fn draw(&self, _: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        if let Some(ref bg) = self.background {
            match bg.as_ref() {
                Background::Color(color) => bg.draw_ex(
                    ctx,
                    DrawParam {
                        dest: Point2::new(0.0, 0.0),
                        color: Some(*color),
                        ..Default::default()
                    },
                )?,
            }
        }
        if let Some(ref dialog) = self.dialog {
            dialog.draw_ex(ctx, DrawParam::default())?;
        }
        Ok(())
    }
}

pub enum Status {
    CommandsApplied,
    PendingCommands,
}
