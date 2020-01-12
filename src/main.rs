mod fly;
mod phys;
mod render;
mod resources;
mod controls;
mod player;
mod ground;

use crate::fly::Fly;

fn main() {
  match Fly::run() {
    Ok(_) => println!("Exiting"),
    Err(e) => println!("Error occured: {}", e)
  }
}
