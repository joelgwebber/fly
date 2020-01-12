use ggez::{
  self, GameResult,
  graphics::{self, Drawable, DrawMode, DrawParam, Mesh},
};
use legion::entity::Entity;
use legion::world::World;
use nalgebra::Vector2;

use crate::camera::{Camera, Renderable, RenderComp};
use crate::fly::{Context, InitContext};
use crate::phys::Physics;
use crate::resources::ResKey;

pub struct Ground {
  rect: ResKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroundComp {
  rect: ResKey,
}

impl Ground {
  pub fn init(ctx: &mut InitContext, camera: &mut Camera) -> GameResult<Ground> {
    camera.register::<GroundComp>();

    let r = Mesh::new_rectangle(
      &mut ctx.gctx,
      DrawMode::fill(),
      graphics::Rect { x: 0., y: 0., w: 500., h: 10. },
      graphics::WHITE,
    )?;

    Ok(Ground {
      rect: ctx.meshes.reg_mesh(r),
    })
  }

  pub fn new(&self, world: &mut World, physics: &mut Physics) -> Entity {
    world.insert((), vec![
      (physics.add_static_rect(Vector2::new(0., 0.), Vector2::new(500., 10.)),
       RenderComp { pos: Vector2::new(0., 0.), rot: 0. },
       GroundComp { rect: self.rect }),
    ])[0]
  }
}

impl Renderable for GroundComp {
  fn render(&self, rend: &RenderComp, ctx: &mut Context) -> GameResult {
    let dp = DrawParam::default()
      .color(graphics::BLACK)
      .rotation(rend.rot)
      .dest([rend.pos.x, rend.pos.y]);

    let rect = ctx.meshes.mesh(self.rect).unwrap();
    rect.draw(&mut ctx.gctx, dp)
  }
}
