extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Debug, Default)]
struct GameConfig {
  save_dir : Option<String>,
  autosave : bool,
  fov      : f32,
  render_distance : u32,
}

fn main() {
  let config = GameConfig::default();
  let json = serde_json::to_string(&config).expect("Could not serialize");
  print!("{}", json);
}