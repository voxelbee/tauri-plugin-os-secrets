const COMMANDS: &[&str] = &["set", "get", "remove"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .build();
}
