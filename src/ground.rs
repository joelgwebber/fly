use ggez::{
  self, GameResult,
  graphics::{self, Drawable, DrawMode, DrawParam, Mesh},
};
use legion::entity::Entity;
use legion::world::World;
use nalgebra::Vector2;

use crate::fly::Context;
use crate::phys::Physics;
use crate::render::RenderComp;
use crate::resources::ResKey;

pub struct Grounds {
  rect: ResKey,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ground {
  rect: ResKey,
}

impl Grounds {
  pub fn init(ctx: &mut Context) -> GameResult<Grounds> {
    let r = Mesh::new_rectangle(
      &mut ctx.gctx,
      DrawMode::fill(),
      graphics::Rect { x: 0., y: 0., w: 500., h: 10. },
      graphics::WHITE,
    )?;

    Ok(Grounds {
      rect: ctx.meshes.reg_mesh(r),
    })
  }

  pub fn new(&self, world: &mut World, physics: &mut Physics) -> Entity {
    world.insert((), vec![
      (physics.add_static_rect(Vector2::new(0., 0.), Vector2::new(500., 10.)),
       RenderComp { pos: Vector2::new(0., 0.), rot: 0. },
       Ground { rect: self.rect }),
    ])[0]
  }
}

impl Ground {
  pub fn draw(&self, rend: &RenderComp, ctx: &mut Context) -> GameResult {
    let dp = DrawParam::default()
      .color(graphics::BLACK)
      .rotation(rend.rot)
      .dest([rend.pos.x, rend.pos.y]);

    let rect = ctx.meshes.mesh(self.rect).unwrap();
    rect.draw(&mut ctx.gctx, dp)
  }
}
