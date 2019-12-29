use std::sync::{Arc, Mutex};

use ggez::{
  Context,
  event::EventHandler,
  GameResult, graphics::{self, DrawParam},
};
use legion::prelude::*;
use nalgebra::Vector2;
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::{
  force_generator::DefaultForceGeneratorSet,
  joint::DefaultJointConstraintSet,
  object::{BodyHandle, BodyPartHandle, ColliderDesc, ColliderHandle, DefaultBodyHandle,
           DefaultBodySet, DefaultColliderHandle, DefaultColliderSet, RigidBodyDesc},
  world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Body {
  bh: DefaultBodyHandle,
  ch: DefaultColliderHandle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Location {
  pos: Vector2<f32>,
  rot: f32,
}

pub struct Fly {
  universe: Universe,
  world: World,
  schedule: Schedule,
  physics: Arc<Mutex<Physics>>,
}

struct Physics {
  mworld: DefaultMechanicalWorld<f32>,
  gworld: DefaultGeometricalWorld<f32>,
  bodies: DefaultBodySet<f32>,
  colliders: DefaultColliderSet<f32>,
  constraints: DefaultJointConstraintSet<f32>,
  forces: DefaultForceGeneratorSet<f32>,
}

impl Fly {
  pub fn new() -> Fly {
    let universe = Universe::new();
    let world = universe.create_world();

    let mworld = DefaultMechanicalWorld::new(Vector2::new(0., -9.81));
    let gworld = DefaultGeometricalWorld::new();
    let bodies = DefaultBodySet::new();
    let colliders = DefaultColliderSet::new();
    let constraints = DefaultJointConstraintSet::new();
    let forces = DefaultForceGeneratorSet::new();

    let physics = Arc::new(Mutex::new(Physics {
      mworld,
      gworld,
      bodies,
      colliders,
      constraints,
      forces,
    }));

    let schedule = Schedule::builder()
      .add_system(physics_system(physics.clone()))
      .add_system(physics_update_system(physics.clone()))
      .build();

    Fly {
      universe,
      world,
      schedule,
      physics,
    }
  }

  pub fn new_ball(&mut self) {
    let mut physics = self.physics.lock().unwrap();
    let body = RigidBodyDesc::new()
      .translation(Vector2::new(20., 20.))
      .build();
    let bh = physics.bodies.insert(body);

    let shape = ShapeHandle::new(Ball::new(10.));
    let collider = ColliderDesc::new(shape)
      .density(1.0)
      .build(BodyPartHandle(bh, 0));
    let ch = physics.colliders.insert(collider);

    self.world.insert((), vec![
      (Body { bh, ch }, Location { pos: Vector2::new(0., 0.), rot: 0. }),
    ]);
  }
}

fn physics_system(amphys: Arc<Mutex<Physics>>) -> Box<dyn Schedulable> {
  SystemBuilder::new("physics")
    .build(move |_, _, _, _| {
      let physics = &mut *amphys.lock().unwrap();
      physics.mworld.step(
        &mut physics.gworld,
        &mut physics.bodies,
        &mut physics.colliders,
        &mut physics.constraints,
        &mut physics.forces,
      );
    })
}

fn physics_update_system(amphys: Arc<Mutex<Physics>>) -> Box<dyn Schedulable> {
  SystemBuilder::new("physics_update")
    .with_query(<(Read<Body>, Write<Location>)>::query())
    .build(move |_, mut world, _, query| {
      let physics = &*amphys.lock().unwrap();
      for (body, mut loc) in query.iter(&mut world) {
        let body = physics.bodies.rigid_body(body.bh).unwrap();
        let v = body.position().translation.vector;
        loc.pos.x = v[0];
        loc.pos.y = v[1];
        println!("{}, {}", loc.pos.x, loc.pos.y);
      }
    })
}

impl EventHandler for Fly {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.schedule.execute(&mut self.world);
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let circle = graphics::Mesh::new_circle(
      ctx,
      graphics::DrawMode::fill(),
      [0., 0.],
      32.0,
      1.0,
      graphics::WHITE,
    )?;

    graphics::clear(ctx, graphics::WHITE);

    for loc in <Read<Location>>::query().iter(&mut self.world) {
      let dp = DrawParam::default()
        .color(graphics::BLACK)
        .dest([loc.pos.x, loc.pos.y]);
      graphics::draw(ctx, &circle, dp)?;
    }

    graphics::present(ctx)
  }

  fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {
    // TODO
  }
}
