use std::collections::HashMap;

use ggez::graphics::Mesh;

pub type MeshKey = u32;

pub struct Meshes {
  cur_key: MeshKey,
  rs: HashMap<MeshKey, Mesh>,
}

impl Meshes {
  pub fn new() -> Meshes {
    Meshes {
      cur_key: 0,
      rs: HashMap::new(),
    }
  }

  pub fn register(&mut self, mesh: Mesh) -> MeshKey {
    let key = self.cur_key;
    self.cur_key += 1;
    self.rs.insert(key, mesh);
    key
  }

  pub fn mesh(&self, key: MeshKey) -> Option<&Mesh> {
    self.rs.get(&key)
  }
}
