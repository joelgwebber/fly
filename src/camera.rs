use ggez::{graphics, Context, GameResult};
use legion::query::IntoQuery;
use legion::query::Read;
use legion::world::World;
use nalgebra::Vector2;

use crate::game::Shared;

#[derive(Clone, Debug, PartialEq)]
pub struct RenderComp {
  pub pos: Vector2<f32>,
  pub rot: f32,
}

pub trait Renderable {
  fn render(&self, shared: &Shared, ctx: &mut Context, rend: &RenderComp) -> GameResult;
}

type RenderFn = fn(&Shared, &mut Context, &mut World) -> GameResult;

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
    self.renderers.push(|shared, ctx, world| {
      for (rend, comp) in <(Read<RenderComp>, Read<R>)>::query().iter(world) {
        comp.render(shared, ctx, &rend)?;
      }
      Ok(())
    })
  }

  pub fn render(&self, shared: &Shared, ctx: &mut Context, world: &mut World) -> GameResult {
    graphics::clear(ctx, graphics::WHITE);

    self.renderers.iter().for_each(|rfn| {
      match rfn(shared, ctx, world) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e)
      }
    });

    graphics::present(ctx)
  }
}
