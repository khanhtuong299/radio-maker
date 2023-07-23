
use std::{fs, path::Path};
use std::sync::Mutex;
use lazy_static::lazy_static;
struct GlobalState {
    song_name: String,
    is_playing:bool,
    is_playable:bool,
    is_coverted:bool,
    covert_name:String
}

lazy_static!{
    static ref GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(GlobalState{
    song_name: String::from("tmp"),
    is_playing:false,
    is_playable:false,
    is_coverted:false,
    covert_name:String::from("tmp_converted")
});
}

pub fn on_input(path: &str) -> &str {
    
    //check file exist and read
    let file_path = Path::new(path);
    let data = match fs::read(file_path) {
        Ok(v) => v,
        Err(_) => {
            println!("File does not exist!");
            return "File does not exist!"
        } ,
    };
    //load file
    let (header, _) =  match puremp3::read_mp3(&data[..]) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid MP3");
            return "Invalid MP3"
        },
    };
    
    println!("Frameheader {:?}", header);
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    
    
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    cur_state.song_name = String::from(file_name);
    cur_state.is_playable = true;
    cur_state.is_playing = false;
    cur_state.is_coverted = false;

    file_name
} 

pub fn play_music() -> String {

    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    
    match cur_state.is_playable {
        true => match  cur_state.is_playing {
            true => {
                cur_state.is_playing = false;
                "Stop".to_string()
            },
            false => {
                cur_state.is_playing = true;
                "Playing".to_string()
        }
        }
        false => "Nothing to play".to_string()
    }
}

pub fn reset_state(){
    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    cur_state.song_name = "".to_string();
    cur_state.is_playable = false;
    cur_state.is_coverted = false;
    cur_state.covert_name = "".to_string();
}