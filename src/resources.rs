use std::collections::HashMap;

use ggez::graphics::Mesh;

pub type ResKey = u32;

pub struct Resources {
  cur_key: ResKey,
  meshes: HashMap<ResKey, Mesh>,
}

impl Resources {
  pub fn new() -> Resources {
    Resources {
      cur_key: 0,
      meshes: HashMap::new(),
    }
  }

  pub fn reg_mesh(&mut self, mesh: Mesh) -> ResKey {
    let key = self.cur_key;
    self.cur_key += 1;
    self.meshes.insert(key, mesh);
    key
  }

  pub fn mesh(&self, key: ResKey) -> Option<&Mesh> {
    self.meshes.get(&key)
  }
}
