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

pub struct Fly {
  universe: Universe,
  world: World,
  schedule: Schedule,
  physics: Physics,

  ground: Ground,
  player: Player,

  controls: Controls,
  shared: Shared,
}

pub struct Shared {
  pub meshes: Meshes,
  pub camera: Camera,
}

impl Fly {
  pub fn run() -> GameResult {
    let mut setup = WindowSetup::default();
    setup.title = "fly".to_string();

    let mut mode = WindowMode::default();
    mode.width = 1024.;
    mode.height = 512.;

    let (mut gctx, mut event_loop) = ContextBuilder::new("fly", "Joel Webber")
      .window_setup(setup)
      .window_mode(mode)
      .build()
      .expect("failed to create context");

    let universe = Universe::new();
    let world = universe.create_world();
    let physics = Physics::new();
    let schedule = Fly::init_systems(&physics);
    let controls = Controls::new();

    let mut shared = Shared {
      meshes: Meshes::new(),
      camera: Camera::new(),
    };
    let ground = Ground::init(&mut shared, &mut gctx)?;
    let player = Player::init(&mut shared, &mut gctx)?;

    let fly = &mut Fly {
      universe,
      world,
      schedule,
      physics,
      ground,
      player,
      controls,
      shared,
    };

    fly.init_scene();
    event::run(&mut gctx, &mut event_loop, fly)
  }

  fn init_systems(physics: &Physics) -> Schedule {
    Schedule::builder()
      .add_system(physics.cmd_system())
      .add_system(physics.sim_system())
      .build()
  }

  fn init_scene(&mut self) {
    self.controls.player = Some(self.player.new(&mut self.world, &mut self.physics));
    self.ground.new(&mut self.world, &mut self.physics);
  }
}

impl EventHandler for Fly {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.controls.update(&mut self.world);
    self.schedule.execute(&mut self.world);
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    self.shared.camera.render(&self.shared, ctx, &mut self.world)
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = true;
  }

  fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = false
  }
}
