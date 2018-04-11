use ggez::{self, GameResult, graphics::{self, BlendMode, DrawMode, DrawParam, Drawable, Point2}};

use engine::{self, color, draw_cache::{DrawCache, TryIntoDrawable}, ui::{Dialog, DialogCache},
             visual_novel::command::{BackgroundCommand, Command}};

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
        self.dialog = Some(DrawCache::new(Dialog {
            text: command.text.clone(),
            portrait: None,
        }));
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
            match bg.get() {
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

enum Background {
    Color(graphics::Color),
}

impl TryIntoDrawable<BackgroundCache> for Background {
    fn try_into_drawable(&self, ctx: &mut ggez::Context) -> GameResult<BackgroundCache> {
        let h = ctx.conf.window_mode.height as f32;
        let w = ctx.conf.window_mode.width as f32;
        let points = [
            Point2::new(0.0, 0.0),
            Point2::new(0.0, h),
            Point2::new(w, h),
            Point2::new(w, 0.0),
        ];
        let mesh = graphics::Mesh::new_polygon(ctx, DrawMode::Fill, &points)?;
        Ok(BackgroundCache::Mesh(mesh))
    }
}

enum BackgroundCache {
    Mesh(graphics::Mesh),
}

impl Drawable for BackgroundCache {
    fn draw_ex(&self, ctx: &mut ggez::Context, mode: DrawParam) -> GameResult<()> {
        match self {
            &BackgroundCache::Mesh(ref mesh) => mesh.draw_ex(ctx, mode),
        }
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        match self {
            &mut BackgroundCache::Mesh(ref mut mesh) => mesh.set_blend_mode(mode),
        }
    }

    fn get_blend_mode(&self) -> Option<BlendMode> {
        match self {
            &BackgroundCache::Mesh(ref mesh) => mesh.get_blend_mode(),
        }
    }
}
