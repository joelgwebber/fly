use std::sync::Arc;

use ggez::{
  self, Context, GameResult,
  graphics::{self, Drawable, DrawParam, Mesh},
};
use legion::entity::Entity;
use legion::world::World;
use nalgebra::Vector2;

use crate::camera::{Camera, Renderable, RenderComp};
use crate::phys::Physics;
use crate::game::Shared;

pub struct Ground {
  res: Arc<Resources>,
}

struct Resources {
  rect: Mesh,
}

#[derive(Clone)]
pub struct GroundComp {
  res: Arc<Resources>,
}

impl Ground {
  pub fn init(shared: &mut Shared, ctx: &mut Context, camera: &mut Camera) -> GameResult<Ground> {
    camera.register::<GroundComp>();

    Ok(Ground {
      res: Arc::new(Resources {
        rect: shared.meshes.rect(ctx, 500., 10.)?,
      }),
    })
  }

  pub fn new(&self, world: &mut World, physics: &mut Physics) -> Entity {
    world.insert((), vec![
      (physics.add_static_rect(Vector2::new(0., 0.), Vector2::new(500., 10.)),
       RenderComp { pos: Vector2::new(0., 0.), rot: 0. },
       GroundComp { res: self.res.clone() }),
    ])[0]
  }
}

impl Renderable for GroundComp {
  fn render(&self, _shared: &Shared, ctx: &mut Context, rend: &RenderComp) -> GameResult {
    let dp = DrawParam::default()
      .color(graphics::BLACK)
      .rotation(rend.rot)
      .dest([rend.pos.x, rend.pos.y]);

    self.res.rect.draw(ctx, dp)
  }
}
