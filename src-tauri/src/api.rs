#[path = "music_processing.rs"] mod music_processing;

#[tauri::command]
pub fn greet(name: &str) -> String {
  println!("received command {}", name);
  format!("Hello, {}!", name)
}

// #[tauri::command]
// pub fn log(level: i32, content: &str){
//   println!("{} level: {}, msg: {}", chrono::offset::Local::now(),
//     level,
//     content);
// }

#[tauri::command]
pub fn on_drop(path: &str) -> String {
  println!("received command on_drop");
  music_processing::reset_state();
  music_processing::on_input(path).to_string()
}

#[tauri::command]
pub fn play_music() -> String {
    music_processing::play_music()
}