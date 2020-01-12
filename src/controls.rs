use legion::entity::Entity;
use legion::world::World;

use crate::phys::{PhysComp, PhysCmd};

pub struct Controls {
  pub player: Option<Entity>,
  pub flapping: bool,
}

impl Controls {
  pub fn new() -> Controls {
    Controls {
      player: None,
      flapping: false,
    }
  }

  pub fn update(&mut self, world: &mut World) {
    if self.player.is_none() {
      return;
    }
    if self.flapping {
      let ent = self.player.unwrap();
      let mut phys = world.get_component_mut::<PhysComp>(ent).unwrap();
      phys.cmd = PhysCmd::Lift(10.);
    }
  }
}
