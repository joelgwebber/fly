use ggez::GameResult;
use ggez::graphics;
use legion::query::IntoQuery;
use legion::query::Read;
use legion::world::World;
use nalgebra::Vector2;

use crate::fly::Context;

#[derive(Clone, Debug, PartialEq)]
pub struct RenderComp {
  pub pos: Vector2<f32>,
  pub rot: f32,
}

pub trait Renderable {
  fn render(&self, rend: &RenderComp, ctx: &mut Context) -> GameResult;
}

type RenderFn = fn(&mut World, &mut Context) -> GameResult;

pub struct Camera {
  renderers: Vec<RenderFn>,
}

impl Camera {
  pub fn new() -> Camera {
    Camera {
      renderers: Vec::new(),
    }
  }

  pub fn register<R>(&mut self)
    where
      R: Renderable + Send + Sync + 'static
  {
    self.renderers.push(|world, ctx| {
      for (rend, comp) in <(Read<RenderComp>, Read<R>)>::query().iter(world) {
        comp.render(&rend, ctx)?;
      }
      Ok(())
    })
  }

  pub fn render(&self, world: &mut World, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx.gctx, graphics::WHITE);

    self.renderers.iter().for_each(|rfn| {
      match rfn(world, ctx) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e)
      }
    });

    graphics::present(ctx.gctx)
  }
}
