use ggez::{
  self, conf::{WindowMode, WindowSetup}, Context, ContextBuilder, event::{self, EventHandler}, GameResult,
};
use ggez::event::MouseButton;
use legion::prelude::*;
use legion::schedule::Schedule;

use crate::camera::Camera;
use crate::controls::Controls;
use crate::ground::Ground;
use crate::meshes::Meshes;
use crate::phys::Physics;
use crate::player::Player;

pub const SCREEN_WIDTH: f32 = 1024.;
pub const SCREEN_HEIGHT: f32 = 512.;

pub struct Fly {
  universe: Universe,
  schedule: Schedule,

  ground: Ground,
  player: Player,

  controls: Controls,
  player_ent: Entity,
  shared: Shared,
}

pub struct Shared {
  pub meshes: Meshes,
  pub world: World,
  pub physics: Physics,
  pub camera: Camera,
}

impl Fly {
  pub fn run() -> GameResult {
    let mut setup = WindowSetup::default();
    setup.title = "fly".to_string();

    let mut mode = WindowMode::default();
    mode.width = SCREEN_WIDTH;
    mode.height = SCREEN_HEIGHT;

    let (mut ctx, mut event_loop) = ContextBuilder::new("fly", "Joel Webber")
      .window_setup(setup)
      .window_mode(mode)
      .build()
      .expect("failed to create context");

    let universe = Universe::new();

    let mut shared = Shared {
      meshes: Meshes::new(),
      world: universe.create_world(),
      physics: Physics::new(),
      camera: Camera::new(),
    };
    let ground = Ground::init(&mut shared, &mut ctx)?;
    let player = Player::init(&mut shared, &mut ctx)?;

    let player_ent = player.new(&mut shared, &mut ctx)?;
    ground.new(&mut shared, &mut ctx)?;

    let controls = Controls::new(player_ent);

    let schedule = Schedule::builder()
      .add_system(shared.physics.cmd_system())
      .add_system(shared.physics.sim_system())
      .build();

    let fly = &mut Fly {
      universe,
      schedule,
      ground,
      player,

      controls,
      player_ent,
      shared,
    };

    event::run(&mut ctx, &mut event_loop, fly)
  }
}

impl EventHandler for Fly {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.controls.update(&mut self.shared);
    self.shared.camera.update(&self.shared.world, self.player_ent);
    self.schedule.execute(&mut self.shared.world);
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    self.shared.camera.render(&mut self.shared.world, ctx)
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = true;
  }

  fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = false
  }
}
