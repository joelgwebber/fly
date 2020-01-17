use std::sync::Arc;

use ggez::{
  self, Context, GameResult,
  graphics::{self, Drawable, DrawParam, Mesh},
};
use legion::entity::Entity;
use legion::world::World;
use nalgebra::Vector2;

use crate::camera::{Renderable, RenderComp};
use crate::fly::Shared;
use crate::phys::Physics;

pub struct Player {
  res: Arc<Resources>,
}

struct Resources {
  circle: Mesh,
}

#[derive(Clone)]
pub struct PlayerComp {
  res: Arc<Resources>,
}

impl Player {
  pub fn init(shared: &mut Shared, ctx: &mut Context) -> GameResult<Player> {
    shared.camera.register::<PlayerComp>();

    Ok(Player {
      res: Arc::new(Resources {
        circle: shared.meshes.circle(ctx, 10.)?,
      }),
    })
  }

  pub fn new(&mut self, world: &mut World, physics: &mut Physics) -> Entity {
    world.insert((), vec![
      (physics.add_ball(Vector2::new(20., 200.), 10.),
       RenderComp { pos: Vector2::new(0., 0.), rot: 0. },
       PlayerComp { res: self.res.clone() /*circle: self.circle*/ }),
    ])[0]
  }
}

impl Renderable for PlayerComp {
  fn render(&self, _shared: &Shared, ctx: &mut Context, rend: &RenderComp) -> GameResult {
    let dp = DrawParam::default()
      .color(graphics::BLACK)
      .rotation(rend.rot)
      .dest([rend.pos.x, rend.pos.y]);

    self.res.circle.draw(ctx, dp)
  }
}
