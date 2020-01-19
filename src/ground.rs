use std::sync::Arc;

use ggez::{
  self, Context, GameResult,
  graphics::{self, Drawable, DrawParam, Mesh},
};
use legion::entity::Entity;
use legion::world::World;
use nalgebra::{Point2, Vector2};

use crate::camera::{Renderable, RenderComp};
use crate::fly::Shared;
use crate::phys::Physics;

pub struct Ground {
  res: Arc<Resources>,
}

struct Resources {}

#[derive(Clone)]
pub struct GroundComp {
  poly: Mesh,
  res: Arc<Resources>,
}

impl Ground {
  pub fn init(shared: &mut Shared, ctx: &mut Context) -> GameResult<Ground> {
    shared.camera.register::<GroundComp>();

    Ok(Ground {
      res: Arc::new(Resources {}),
    })
  }

  pub fn new(&self, shared: &mut Shared, ctx: &mut ggez::Context) -> GameResult<Entity> {
    let points: [Point2<f32>; 4] = [
      Point2::new(0., 50.),
      Point2::new(100., 10.),
      Point2::new(100., 0.),
      Point2::new(0., 0.),
    ];

    Ok(shared.world.insert((), vec![
      (shared.physics.add_static_poly(&points),
       RenderComp { pos: Vector2::new(0., 0.), rot: 0. },
       GroundComp {
         res: self.res.clone(),
         poly: shared.meshes.poly(ctx, &points)?,
       }),
    ])[0])
  }
}

impl Renderable for GroundComp {
  fn render(&self, ctx: &mut Context, rend: &RenderComp) -> GameResult {
    let dp = DrawParam::default()
      .color(graphics::BLACK)
      .rotation(rend.rot)
      .dest([rend.pos.x, rend.pos.y]);

    self.poly.draw(ctx, dp)
  }
}
