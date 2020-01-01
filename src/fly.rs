use std::ops::Deref;
use std::sync::{Arc, Mutex};

use ggez::{
  Context,
  event::EventHandler,
  GameResult, graphics::{self, DrawParam},
};
use legion::prelude::*;
use nalgebra::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::{
  force_generator::DefaultForceGeneratorSet,
  joint::DefaultJointConstraintSet,
  math::Velocity,
  object::{BodyPartHandle, BodyStatus, ColliderDesc, DefaultBodyHandle, DefaultBodySet,
           DefaultColliderHandle, DefaultColliderSet, RigidBodyDesc},
  world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};

use crate::renderers::{self, Drawables, Key};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Body {
  bh: DefaultBodyHandle,
  ch: DefaultColliderHandle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Render {
  pos: Vector2<f32>,
  rot: f32,
  draw: Key,
}

pub struct Fly {
  universe: Universe,
  world: World,
  schedule: Schedule,
  physics: Arc<Mutex<Physics>>,
  drawables: renderers::Drawables,
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
  pub fn new() -> Self {
    let universe = Universe::new();
    let world = universe.create_world();

    let physics = Arc::new(Mutex::new(Physics {
      mworld: DefaultMechanicalWorld::new(Vector2::new(0., -9.81)),
      gworld: DefaultGeometricalWorld::new(),
      bodies: DefaultBodySet::new(),
      colliders: DefaultColliderSet::new(),
      constraints: DefaultJointConstraintSet::new(),
      forces: DefaultForceGeneratorSet::new(),
    }));

    let schedule = Schedule::builder()
      .add_system(physics_system(physics.clone()))
      .build();

    Fly {
      universe,
      world,
      schedule,
      physics,
      drawables: Drawables::new(),
    }
  }

  pub fn new_ball(&mut self, ctx: &mut Context) -> GameResult<&[Entity]> {
    let mut physics = self.physics.lock().unwrap();

    let body = RigidBodyDesc::new()
      .translation(Vector2::new(100., 50.))
      .velocity(Velocity::linear(10., 0.))
      .build();
    let bh = physics.bodies.insert(body);

    let shape = ShapeHandle::new(Ball::new(10.));
    let collider = ColliderDesc::new(shape)
      .density(1.0)
      .build(BodyPartHandle(bh, 0));
    let ch = physics.colliders.insert(collider);

    Ok(self.world.insert((), vec![
      (Body { bh, ch }, Render { pos: Vector2::new(0., 0.), rot: 0., draw: self.drawables.player(ctx)? }),
    ]))
  }

  pub fn new_ground(&mut self, ctx: &mut Context) -> GameResult<&[Entity]> {
    let mut physics = self.physics.lock().unwrap();

    let body = RigidBodyDesc::new()
      .translation(Vector2::new(0., 0.))
      .set_status(BodyStatus::Static)
      .build();
    let bh = physics.bodies.insert(body);

    let shape = ShapeHandle::new(Cuboid::new(Vector2::new(500., 10.)));
    let collider = ColliderDesc::new(shape)
      .density(1.0)
      .build(BodyPartHandle(bh, 0));
    let ch = physics.colliders.insert(collider);

    Ok(self.world.insert((), vec![
      (Body { bh, ch }, Render { pos: Vector2::new(0., 0.), rot: 0., draw: self.drawables.ground(ctx)? }),
    ]))
  }
}

fn physics_system(amphys: Arc<Mutex<Physics>>) -> Box<dyn Schedulable> {
  SystemBuilder::new("physics")
    .with_query(<(Read<Body>, Write<Render>)>::query())
    .build(move |_, mut world, _, query| {
      let physics = &mut *amphys.lock().unwrap();
      physics.mworld.step(
        &mut physics.gworld,
        &mut physics.bodies,
        &mut physics.colliders,
        &mut physics.constraints,
        &mut physics.forces,
      );

      for (body, mut rend) in query.iter(&mut world) {
        let body = physics.bodies.rigid_body(body.bh).unwrap();
        let v = body.position().translation.vector;
        rend.pos.x = v[0];
        rend.pos.y = v[1];
        rend.rot = body.position().rotation.angle();
      }
    })
}

impl EventHandler for Fly {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    self.schedule.execute(&mut self.world);
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, graphics::WHITE);

    for rend in <(Read<Render>)>::query().iter(&mut self.world) {
      let dp = DrawParam::default()
        .color(graphics::BLACK)
        .rotation(rend.rot)
        .dest([rend.pos.x, rend.pos.y]);
      let drawable = self.drawables.drawable(&rend.draw).unwrap().deref();
      drawable.draw(ctx, dp)?;
    }

    graphics::present(ctx)
  }

  fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {
    // TODO
  }
}
