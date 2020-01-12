use ggez::GameResult;
use ggez::graphics;
use legion::query::IntoQuery;
use legion::query::Read;
use legion::world::World;
use nalgebra::Vector2;

use crate::fly::Context;
use crate::ground::Ground;
use crate::player::Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderComp {
  pub pos: Vector2<f32>,
  pub rot: f32,
}

pub fn render_world(world: &mut World, ctx: &mut Context) -> GameResult {
  graphics::clear(ctx.gctx, graphics::WHITE);

  {
    for (ground, rend) in <(Read<Ground>, Read<RenderComp>)>::query().iter(world) {
      ground.draw(&rend, ctx)?;
    }

    for (player, rend) in <(Read<Player>, Read<RenderComp>)>::query().iter(world) {
      player.draw(&rend, ctx)?;
    }
  }

  graphics::present(ctx.gctx)
}
