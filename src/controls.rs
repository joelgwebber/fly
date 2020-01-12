use legion::entity::Entity;
use legion::world::World;

use crate::phys::{PhysCmd, PhysComp};

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
    match self.player {
      Some(player) => {
        if self.flapping {
          let mut phys = world.get_component_mut::<PhysComp>(player).unwrap();
          phys.cmd = PhysCmd::Lift(10.);
        }
      }
      None => {}
    }
  }
}
