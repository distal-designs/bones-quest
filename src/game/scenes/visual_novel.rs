use ggez::{self, GameResult};
use ggez::graphics::{DrawParam, Drawable, Point2};

use engine::{self, color};
use engine::draw_cache::DrawCache;
use engine::ui::{Background, BackgroundCache, Dialog, DialogCache, Portrait};
use engine::visual_novel::command::{BackgroundCommand, Command, PortraitCommand};

pub struct VisualNovel {
    commands: Vec<Command>,
    command_index: usize,
    dialog: Option<DrawCache<Dialog, DialogCache>>,
    background: Option<DrawCache<Background, BackgroundCache>>,
    status: Status,
}

impl VisualNovel {
    fn apply(&mut self) {
        let commands = &mut self.commands;
        let command = &mut commands[self.command_index];

        let portrait = match &command.portrait {
            &Some(PortraitCommand::Show(ref character, ref style)) => Some(Portrait {
                character: character.clone(),
                style: style.clone(),
            }),
            &None => match self.dialog {
                Some(ref dialog_draw_cache) => dialog_draw_cache.as_ref().portrait.clone(),
                _ => None,
            },
            &Some(PortraitCommand::Hide) => None,
        };
        let text = command.text.clone();
        self.dialog = Some(DrawCache::new(Dialog { text, portrait }));

        self.background = match command.background {
            Some(BackgroundCommand::Hide) => None,
            Some(BackgroundCommand::Color(ref hex)) => {
                Some(DrawCache::new(Background::Color(color::from_hex(&hex))))
            }
            _ => unimplemented!(),
        };
    }

    pub fn new(commands: Vec<Command>) -> Self {
        Self {
            commands,
            command_index: 0,
            status: Status::PendingCommands,
            dialog: None,
            background: None,
        }
    }
}

impl<I, F> engine::scene::Scene<I, F> for VisualNovel {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        if let Status::PendingCommands = self.status {
            self.apply();
            self.status = Status::CommandsApplied;
        }
        Ok(())
    }

    fn draw(&self, _: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        if let Some(ref bg) = self.background {
            match bg.as_ref() {
                &Background::Color(color) => {
                    bg.draw_ex(
                        ctx,
                        DrawParam {
                            dest: Point2::new(0.0, 0.0),
                            color: Some(color),
                            ..Default::default()
                        },
                    )?;
                }
            };
        };
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
