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

pub struct Player {
  circle: ResKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerComp {
  circle: ResKey,
}

impl Player {
  pub fn init(ctx: &mut InitContext, camera: &mut Camera) -> GameResult<Player> {
    camera.register::<PlayerComp>();

    let c = Mesh::new_circle(
      ctx.gctx,
      DrawMode::fill(),
      [0., 0.],
      10.0,
      1.0,
      graphics::WHITE,
    )?;
    Ok(Player {
      circle: ctx.meshes.reg_mesh(c),
    })
  }

  pub fn new(&mut self, world: &mut World, physics: &mut Physics) -> Entity {
    world.insert((), vec![
      (physics.add_ball(Vector2::new(20., 200.), 10.),
       RenderComp { pos: Vector2::new(0., 0.), rot: 0. },
       PlayerComp { circle: self.circle }),
    ])[0]
  }
}

impl Renderable for PlayerComp {
  fn render(&self, rend: &RenderComp, ctx: &mut Context) -> GameResult {
    let dp = DrawParam::default()
      .color(graphics::BLACK)
      .rotation(rend.rot)
      .dest([rend.pos.x, rend.pos.y]);

    let circle = ctx.meshes.mesh(self.circle).unwrap();
    circle.draw(&mut ctx.gctx, dp)
  }
}
