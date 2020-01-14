use crate::meshes::Meshes;

pub struct Shared {
  pub meshes: Meshes,
}

impl Shared {
  pub fn new() -> Shared {
    Shared {
      meshes: Meshes::new(),
    }
  }
}
