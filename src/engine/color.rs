use ggez::graphics::{get_color, set_color, Color};
use ggez::{Context, GameResult};

pub fn with_color<F>(ctx: &mut Context, color: &Color, fun: F) -> GameResult<()>
where
    F: FnOnce(&mut Context) -> GameResult<()>,
{
    let current_color = get_color(ctx);
    set_color(ctx, *color)?;
    fun(ctx)?;
    set_color(ctx, current_color)?;
    Ok(())
}
