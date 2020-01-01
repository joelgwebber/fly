use std::collections::HashMap;

use ggez::{
  Context,
  GameResult, graphics::{self, Drawable, DrawMode, DrawParam, Mesh},
};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
enum Type {
  Player,
  Ground,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Key(Type, usize);

pub struct Drawables {
  map: HashMap<Key, Box<dyn Drawable>>,
}

impl Drawables {
  pub fn new() -> Drawables {
    Drawables {
      map: HashMap::new(),
    }
  }

  pub fn drawable(&mut self, key: &Key) -> Option<&Box<dyn Drawable>> {
    self.map.get(key)
  }

  pub fn player(&mut self, ctx: &mut Context) -> GameResult<Key> {
    let key = Key(Type::Player, 0);
    if !self.map.contains_key(&key) {
      let circle = Box::new(Mesh::new_circle(
        ctx,
        DrawMode::fill(),
        [0., 0.],
        10.0,
        1.0,
        graphics::WHITE,
      )?);
      self.map.insert(key, circle);
    }
    Ok(key)
  }

  pub fn ground(&mut self, ctx: &mut Context) -> GameResult<Key> {
    let key = Key(Type::Ground, 0);
    if !self.map.contains_key(&key) {
      let rect = Box::new(Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        graphics::Rect{x: 0., y: 0., w: 500., h: 10.},
        graphics::WHITE,
      )?);
      self.map.insert(key, rect);
    }
    Ok(key)
  }
}
