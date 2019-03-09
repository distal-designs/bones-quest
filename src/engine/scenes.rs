pub mod visual_novel;


use ggez::{self, GameResult};

use crate::engine;


pub struct _Overworld;


impl<I, F> engine::scene::Scene<I, F> for _Overworld {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }


    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}


pub struct _Conquest;


impl<I, F> engine::scene::Scene<I, F> for _Conquest {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }


    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}


pub struct _Investigation;


impl<I, F> engine::scene::Scene<I, F> for _Investigation {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }


    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}
