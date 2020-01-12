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

pub struct Players {
  circle: ResKey,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
  circle: ResKey,
}

impl Players {
  pub fn init(ctx: &mut Context) -> GameResult<Players> {
    let c = Mesh::new_circle(
      ctx.gctx,
      DrawMode::fill(),
      [0., 0.],
      10.0,
      1.0,
      graphics::WHITE,
    )?;
    Ok(Players {
      circle: ctx.meshes.reg_mesh(c),
    })
  }

  pub fn new(&mut self, world: &mut World, physics: &mut Physics) -> Entity {
    world.insert((), vec![
      (physics.add_ball(Vector2::new(20., 200.), 10.),
       RenderComp { pos: Vector2::new(0., 0.), rot: 0. },
       Player { circle: self.circle }),
    ])[0]
  }
}

impl Player {
  pub fn draw(&self, rend: &RenderComp, ctx: &mut Context) -> GameResult {
    let dp = DrawParam::default()
      .color(graphics::BLACK)
      .rotation(rend.rot)
      .dest([rend.pos.x, rend.pos.y]);

    let circle = ctx.meshes.mesh(self.circle).unwrap();
    circle.draw(&mut ctx.gctx, dp)
  }
}
