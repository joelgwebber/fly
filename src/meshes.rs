use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, Mesh};

pub struct Meshes {
}

impl Meshes {
  pub fn new() -> Meshes {
    Meshes { }
  }

  pub fn circle(&self, gctx: &mut Context, radius: f32) -> GameResult<Mesh> {
    Mesh::new_circle(
      gctx,
      DrawMode::fill(),
      [0., 0.],
      radius,
      1.0,
      graphics::WHITE,
    )
  }

  pub fn rect(&self, gctx: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
    Mesh::new_rectangle(
      gctx,
      DrawMode::fill(),
      graphics::Rect { x: 0., y: 0., w: width, h: height },
      graphics::WHITE,
    )
  }
}
