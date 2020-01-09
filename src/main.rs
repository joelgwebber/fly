mod fly;
mod meshes;
mod player;
mod ground;
mod phys;

use crate::fly::Fly;

fn main() {
  match Fly::run() {
    Ok(_) => println!("Exiting"),
    Err(e) => println!("Error occured: {}", e)
  }
}
