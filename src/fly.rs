use ggez::{
  self, conf::{WindowMode, WindowSetup}, ContextBuilder, event::{self, EventHandler}, GameResult,
};
use ggez::event::MouseButton;
use legion::prelude::*;

use crate::camera::Camera;
use crate::controls::Controls;
use crate::ground::Ground;
use crate::phys::Physics;
use crate::player::Player;
use crate::resources::Resources;

pub struct Fly {
  universe: Universe,
  world: World,
  schedule: Schedule,
  physics: Physics,

  ground: Ground,
  player: Player,

  camera: Camera,
  controls: Controls,

  pub resources: Resources,
}

pub struct InitContext<'a> {
  pub gctx: &'a mut ggez::Context,
  pub meshes: &'a mut Resources,
}

pub struct Context<'a> {
  pub gctx: &'a mut ggez::Context,
  pub meshes: &'a Resources,
}

impl Fly {
  pub fn run() -> GameResult {
    let mut setup = WindowSetup::default();
    setup.title = "fly".to_string();

    let mut mode = WindowMode::default();
    mode.width = 512.;
    mode.height = 512.;

    let (mut gctx, mut event_loop) = ContextBuilder::new("fly", "Joel Webber")
      .window_setup(setup)
      .window_mode(mode)
      .build()
      .expect("failed to create context");

    let universe = Universe::new();
    let world = universe.create_world();
    let mut physics = Physics::new();

    let schedule = Schedule::builder()
      .add_system(physics.cmd_system())
      .add_system(physics.sim_system())
      .build();

    let mut meshes = Resources::new();
    let ctx = &mut InitContext {
      gctx: &mut gctx,
      meshes: &mut meshes,
    };

    let mut camera = Camera::new();
    let ground = Ground::init(ctx, &mut camera)?;
    let player = Player::init(ctx, &mut camera)?;

    let fly = &mut Fly {
      universe,
      world,
      schedule,
      physics,
      ground,
      player,
      camera,
      controls: Controls::new(),
      resources: meshes,
    };

    fly.init_scene();
    event::run(&mut gctx, &mut event_loop, fly)
  }

  fn init_scene(&mut self) {
    self.controls.player = Some(self.player.new(&mut self.world, &mut self.physics));
    self.ground.new(&mut self.world, &mut self.physics);
  }
}

impl EventHandler for Fly {
  fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<()> {
    self.controls.update(&mut self.world);
    self.schedule.execute(&mut self.world);
    Ok(())
  }

  fn draw(&mut self, gctx: &mut ggez::Context) -> GameResult {
    let ctx = &mut Context { meshes: &mut self.resources, gctx };
    self.camera.render(&mut self.world, ctx)
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = true;
  }

  fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = false
  }
}
