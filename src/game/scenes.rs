use ggez::{
    self,
    graphics::{
        self,
        BlendMode,
        Drawable,
        DrawMode,
        DrawParam,
        Point2,
    },
    GameResult,
};

use engine::{
    self,
    draw_cache::{
        TryIntoDrawable,
    },
    color::{
        self,
        with_color,
    },
    visual_novel::command::{
        BackgroundCommand,
        Command,
    },
    ui::Message,
};

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
        let &Background::Color(color) = self;
        with_color(ctx, &color, |ctx| {
            let mesh = graphics::Mesh::new_polygon(ctx, DrawMode::Fill, &points)?;
            Ok(BackgroundCache::Mesh(mesh))
        })
    }
}

enum BackgroundCache {
    Mesh(graphics::Mesh),
}

impl Drawable for BackgroundCache {
    fn draw_ex(&self, ctx: &mut ggez::Context, mode: DrawParam) -> GameResult<()> {
        match self {
            &BackgroundCache::Mesh(ref mesh) => mesh.draw_ex(ctx, mode)
        }
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        match self {
            &mut BackgroundCache::Mesh(ref mut mesh) => mesh.set_blend_mode(mode)
        }
    }

    fn get_blend_mode(&self) -> Option<BlendMode> {
        match self {
            &BackgroundCache::Mesh(ref mesh) => mesh.get_blend_mode()
        }
    }
}

pub struct VisualNovel {
    dialog: Vec<Command>,
    dialog_index: usize,
    message: Option<Message>,
    background: Option<Background>,
    status: Status,
}

impl VisualNovel {
    fn apply(&mut self) {
        let commands = &mut self.dialog;
        let command = &mut commands[self.dialog_index];
        self.message = Some(Message::new(&command.text));
        self.background = match command.background {
            Some(BackgroundCommand::Hide) => None,
            Some(BackgroundCommand::Color(ref hex)) => {
                Some(Background::Color(color::from_hex(&hex)))
            }
            _ => unimplemented!(),
        };
    }

    pub fn new(dialog: Vec<Command>) -> Self {
        Self {
            dialog,
            dialog_index: 0,
            status: Status::PendingCommands,
            message: None,
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
        if let Some(ref message) = self.message {
            message.draw(ctx)?;
        }
        Ok(())
    }
}

pub struct Fight;

impl<I, F> engine::scene::Scene<I, F> for Fight {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}

pub struct Overworld;

impl<I, F> engine::scene::Scene<I, F> for Overworld {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}

pub struct Conquest;

impl<I, F> engine::scene::Scene<I, F> for Conquest {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}

pub struct Investigation;

impl<I, F> engine::scene::Scene<I, F> for Investigation {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }

    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}
