use legion::entity::Entity;
use legion::world::World;

use crate::phys::{PhysCmd, PhysComp};
use crate::fly::Shared;

pub struct Controls {
  pub player_ent: Entity,
  pub flapping: bool,
}

impl Controls {
  pub fn new(player_ent: Entity) -> Controls {
    Controls {
      player_ent,
      flapping: false,
    }
  }

  pub fn update(&mut self, shared: &mut Shared) {
    if self.flapping {
      let mut phys = shared.world.get_component_mut::<PhysComp>(self.player_ent).unwrap();
      phys.cmd = PhysCmd::Lift(10.);
    }
  }
}
