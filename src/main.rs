use ggez::{
  conf::{
    WindowMode, WindowSetup,
  },
  ContextBuilder,
  event,
};

mod fly;
mod renderers;

fn main() {
  let mut setup = WindowSetup::default();
  setup.title = "fly".to_string();

  let mut mode = WindowMode::default();
  mode.width = 512.;
  mode.height = 512.;

  let (mut ctx, mut event_loop) = ContextBuilder::new("fly", "Joel Webber")
    .window_setup(setup)
    .window_mode(mode)
    .build()
    .expect("failed to create context");

  let mut fly = fly::Fly::new();
  fly.new_ball(&mut ctx);
  fly.new_ground(&mut ctx);

  match event::run(&mut ctx, &mut event_loop, &mut fly) {
    Ok(_) => println!("Exiting"),
    Err(e) => println!("Error occured: {}", e)
  }
}
