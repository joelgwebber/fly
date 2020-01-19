use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, Mesh};
use nalgebra::{Vector2, Point2};

pub struct Meshes {
}

impl Meshes {
  pub fn new() -> Meshes {
    Meshes { }
  }

  pub fn circle(&self, ctx: &mut Context, radius: f32) -> GameResult<Mesh> {
    Mesh::new_circle(
      ctx,
      DrawMode::fill(),
      [0., 0.],
      radius,
      1.0,
      graphics::WHITE,
    )
  }

  pub fn rect(&self, ctx: &mut Context, width: f32, height: f32) -> GameResult<Mesh> {
    Mesh::new_rectangle(
      ctx,
      DrawMode::fill(),
      graphics::Rect { x: 0., y: 0., w: width, h: height },
      graphics::WHITE,
    )
  }

  pub fn poly(&self, ctx: &mut Context, points: &[Point2<f32>]) -> GameResult<Mesh> {
    Mesh::new_polygon(ctx, DrawMode::fill(), points, graphics::WHITE)
  }
}
