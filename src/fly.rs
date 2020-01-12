use ggez::{
  self, conf::{WindowMode, WindowSetup}, ContextBuilder, event::{self, EventHandler},
  GameResult,
  graphics,
};
use ggez::event::MouseButton;
use legion::prelude::*;

use crate::controls::Controls;
use crate::ground::Grounds;
use crate::phys::Physics;
use crate::player::Players;
use crate::render::render_world;
use crate::resources::Resources;

pub struct Fly {
  universe: Universe,
  world: World,
  schedule: Schedule,
  physics: Physics,

  grounds: Grounds,
  players: Players,

  controls: Controls,

  pub resources: Resources,
}

pub struct Context<'a> {
  pub gctx: &'a mut ggez::Context,
  pub meshes: &'a mut Resources,
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
    let ctx = &mut Context {
      gctx: &mut gctx,
      meshes: &mut meshes,
    };

    let grounds = Grounds::init(ctx)?;
    let players = Players::init(ctx)?;
    let fly = &mut Fly {
      universe,
      world,
      schedule,
      physics,
      grounds,
      players,
      controls: Controls::new(),
      resources: meshes,
    };

    fly.init_scene();
    event::run(&mut gctx, &mut event_loop, fly)
  }

  fn init_scene(&mut self) {
    self.controls.player = Some(self.new_player());
    self.new_ground();
  }

  fn new_player(&mut self) -> Entity {
    self.players.new(&mut self.world, &mut self.physics)
  }

  fn new_ground(&mut self) -> Entity {
    self.grounds.new(&mut self.world, &mut self.physics)
  }
}

impl EventHandler for Fly {
  fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult<()> {
    self.controls.update(&mut self.world);
    self.schedule.execute(&mut self.world);
    Ok(())
  }

  fn draw(&mut self, gctx: &mut ggez::Context) -> GameResult {
    render_world(&mut self.world, &mut Context { meshes: &mut self.resources, gctx })
  }

  fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = true;
  }

  fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, _button: MouseButton, _x: f32, _y: f32) {
    self.controls.flapping = false
  }

  fn resize_event(&mut self, _ctx: &mut ggez::Context, _width: f32, _height: f32) {
    // TODO
  }
}
