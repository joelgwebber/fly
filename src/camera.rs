use ggez::{Context, GameResult, graphics};
use ggez::graphics::DrawParam;
use legion::entity::Entity;
use legion::query::{IntoQuery, Write};
use legion::query::Read;
use legion::schedule::Schedulable;
use legion::system::SystemBuilder;
use legion::world::World;
use nalgebra::Matrix4;
use nalgebra::Vector2;
use nalgebra::Vector3;

use crate::fly::Shared;
use crate::player::PlayerComp;

pub trait Renderable {
  fn render(&self, ctx: &mut Context, rend: &RenderComp) -> GameResult;
}

type RenderFn = fn(&mut World, &mut Context) -> GameResult;

pub struct Camera {
  renderers: Vec<RenderFn>,
  pub pos: Vector2<f32>,
}

#[derive(Clone)]
pub struct RenderComp {
  pub pos: Vector2<f32>,
  pub rot: f32,
}

impl Camera {
  pub fn new() -> Camera {
    Camera {
      renderers: Vec::new(),
      pos: Vector2::new(0., 0.),
    }
  }

  pub fn register<R>(&mut self)
    where
      R: Renderable + Send + Sync + 'static
  {
    self.renderers.push(|world, ctx| {
      for (rend, comp) in <(Read<RenderComp>, Read<R>)>::query().iter(world) {
        comp.render(ctx, &rend)?;
      }
      Ok(())
    })
  }

  pub fn render(&self, world: &mut World, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::WHITE);

    let m = Matrix4::new_nonuniform_scaling(&Vector3::new(1., -1., 0.))
      .append_translation(&Vector3::new(512., 256., 0.))
      .append_translation(&Vector3::new(-self.pos.x, self.pos.y, 0.));

    graphics::push_transform(ctx, Some(m));
    graphics::apply_transformations(ctx)?;

    self.renderers.iter().for_each(|rfn| {
      match rfn(world, ctx) {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e)
      }
    });

    graphics::pop_transform(ctx);
    graphics::apply_transformations(ctx)?;
    graphics::present(ctx)
  }

  pub fn update(&mut self, world: &World, player_ent: Entity) {
    self.pos = world.get_component::<RenderComp>(player_ent).unwrap().pos;
  }
}
