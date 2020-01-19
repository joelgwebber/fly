use std::sync::{Arc, Mutex};

use legion::query::{Read, Write};
use legion::query::IntoQuery;
use legion::schedule::Schedulable;
use legion::system::SystemBuilder;
use nalgebra::{Vector2, Point2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle, ConvexPolygon};
use nphysics2d::{
  force_generator::DefaultForceGeneratorSet,
  joint::DefaultJointConstraintSet,
  object::{Body, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle, DefaultColliderSet},
  world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};
use nphysics2d::algebra::{Force2, ForceType};
use nphysics2d::object::{BodyPartHandle, BodyStatus, ColliderDesc, RigidBodyDesc};

use crate::camera::RenderComp;

#[derive(Clone, Debug, PartialEq)]
pub enum PhysCmd {
  None,
  Lift(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PhysComp {
  bh: DefaultBodyHandle,
  ch: DefaultColliderHandle,

  pub cmd: PhysCmd,
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

  pub fn cmd_system(&self) -> Box<dyn Schedulable> {
    let mutex = self.state.clone();
    SystemBuilder::new("physics-cmd")
      .with_query(<(Write<PhysComp>)>::query())
      .build(move |_, mut world, _, query| {
        let state = &mut *mutex.lock().unwrap();
        for mut phys in query.iter(&mut world) {
          match phys.cmd {
            PhysCmd::None => {}
            PhysCmd::Lift(amt) => {
              let rigid = state.bodies.rigid_body_mut(phys.bh).unwrap();
              rigid.apply_force(0, &Force2::linear(Vector2::new(amt, amt)), ForceType::Impulse, false);
            }
          }
          phys.cmd = PhysCmd::None;
        }
      })
  }

  pub fn sim_system(&self) -> Box<dyn Schedulable> {
    let mutex = self.state.clone();
    SystemBuilder::new("physics-sim")
      .with_query(<(Read<PhysComp>, Write<RenderComp>)>::query())
      .build(move |_, mut world, _, query| {
        // Step the nphysics world.
        let state = &mut *mutex.lock().unwrap();
        state.mworld.step(
          &mut state.gworld,
          &mut state.bodies,
          &mut state.colliders,
          &mut state.constraints,
          &mut state.forces,
        );

        // Update all the render components from their nphysics bodies.
        for (phys, mut rend) in query.iter(&mut world) {
          let rigid = state.bodies.rigid_body(phys.bh).unwrap();
          let v = rigid.position().translation.vector;
          rend.pos.x = v[0];
          rend.pos.y = v[1];
          rend.rot = rigid.position().rotation.angle();
        }
      })
  }

  pub fn add_static_rect(&mut self, pos: Vector2<f32>, half_extents: Vector2<f32>) -> PhysComp {
    let state = &mut *self.state.lock().unwrap();
    let body = RigidBodyDesc::new()
      .translation(pos)
      .set_status(BodyStatus::Static)
      .build();
    let bh = state.bodies.insert(body);

    let shape = ShapeHandle::new(Cuboid::new(half_extents));
    let collider = ColliderDesc::new(shape)
      .density(0.1)
      .build(BodyPartHandle(bh, 0));
    let ch = state.colliders.insert(collider);

    PhysComp { bh, ch, cmd: PhysCmd::None }
  }

  pub fn add_static_poly(&mut self, points: &[Point2<f32>]) -> PhysComp {
    let state = &mut *self.state.lock().unwrap();
    let body = RigidBodyDesc::new()
      .set_status(BodyStatus::Static)
      .build();
    let bh = state.bodies.insert(body);

    let shape = ShapeHandle::new(ConvexPolygon::try_from_points(points).unwrap());
    let collider = ColliderDesc::new(shape)
      .density(0.1)
      .build(BodyPartHandle(bh, 0));
    let ch = state.colliders.insert(collider);

    PhysComp { bh, ch, cmd: PhysCmd::None }
  }

  pub fn add_ball(&mut self, pos: Vector2<f32>, radius: f32) -> PhysComp {
    let state = &mut *self.state.lock().unwrap();
    let body = RigidBodyDesc::new()
      .translation(pos)
      .build();
    let bh = state.bodies.insert(body);

    let shape = ShapeHandle::new(Ball::new(radius));
    let collider = ColliderDesc::new(shape)
      .density(0.1)
      .build(BodyPartHandle(bh, 0));
    let ch = state.colliders.insert(collider);

    PhysComp { bh, ch, cmd: PhysCmd::None }
  }
}
