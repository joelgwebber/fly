use std::sync::{Arc, Mutex};

use legion::query::{Read, Write};
use legion::query::IntoQuery;
use legion::schedule::Schedulable;
use legion::system::SystemBuilder;
use nalgebra::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::{
  force_generator::DefaultForceGeneratorSet,
  joint::DefaultJointConstraintSet,
  object::{DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle, DefaultColliderSet},
  world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};
use nphysics2d::object::{BodyPartHandle, BodyStatus, ColliderDesc, RigidBodyDesc};

use crate::fly::Render;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Body {
  pub bh: DefaultBodyHandle,
  pub ch: DefaultColliderHandle,
}

pub struct Physics {
  state: Arc<Mutex<State>>,
}

struct State {
  pub mworld: DefaultMechanicalWorld<f32>,
  pub gworld: DefaultGeometricalWorld<f32>,
  pub bodies: DefaultBodySet<f32>,
  pub colliders: DefaultColliderSet<f32>,
  pub constraints: DefaultJointConstraintSet<f32>,
  pub forces: DefaultForceGeneratorSet<f32>,
}

impl Physics {
  pub fn new() -> Physics {
    let state = Arc::new(Mutex::new(State {
      mworld: DefaultMechanicalWorld::new(Vector2::new(0., -9.81)),
      gworld: DefaultGeometricalWorld::new(),
      bodies: DefaultBodySet::new(),
      colliders: DefaultColliderSet::new(),
      constraints: DefaultJointConstraintSet::new(),
      forces: DefaultForceGeneratorSet::new(),
    }));

    Physics {
      state,
    }
  }

  pub fn system(&mut self) -> Box<dyn Schedulable> {
    let state = self.state.clone();
    SystemBuilder::new("physics")
      .with_query(<(Read<Body>, Write<Render>)>::query())
      .build(move |_, mut world, _, query| {
        // Step the nphysics world.
        let state = &mut *state.lock().unwrap();
        state.mworld.step(
          &mut state.gworld,
          &mut state.bodies,
          &mut state.colliders,
          &mut state.constraints,
          &mut state.forces,
        );

        // Update all the render components from their nphysics bodies.
        for (body, mut rend) in query.iter(&mut world) {
          let body = state.bodies.rigid_body(body.bh).unwrap();
          let v = body.position().translation.vector;
          rend.pos.x = v[0];
          rend.pos.y = v[1];
          rend.rot = body.position().rotation.angle();
        }
      })
  }

  pub fn add_static_rect(&mut self, pos: Vector2<f32>, half_extents: Vector2<f32>) -> Body {
    let state = &mut *self.state.lock().unwrap();
    let body = RigidBodyDesc::new()
      .translation(pos)
      .set_status(BodyStatus::Static)
      .build();
    let bh = state.bodies.insert(body);

    let shape = ShapeHandle::new(Cuboid::new(half_extents));
    let collider = ColliderDesc::new(shape)
      .density(1.0)
      .build(BodyPartHandle(bh, 0));
    let ch = state.colliders.insert(collider);

    Body { bh, ch }
  }

  pub fn add_ball(&mut self, pos: Vector2<f32>, radius: f32) -> Body {
    let state = &mut *self.state.lock().unwrap();
    let body = RigidBodyDesc::new()
      .translation(pos)
      .build();
    let bh = state.bodies.insert(body);

    let shape = ShapeHandle::new(Ball::new(radius));
    let collider = ColliderDesc::new(shape)
      .density(1.0)
      .build(BodyPartHandle(bh, 0));
    let ch = state.colliders.insert(collider);

    Body { bh, ch }
  }
}
