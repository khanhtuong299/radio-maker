
use std::{fs, path::Path, fs::File};
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::thread;

struct GlobalState {
    song_name: String,
    path: String,
    is_playing:bool,
    is_playable:bool,
    is_coverted:bool,
    covert_name:String,
    // sink: Sink
}

lazy_static!{
    static ref GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(GlobalState{
        song_name: String::from("tmp"),
        path: String::from("./"),
        is_playing:false,
        is_playable:false,
        is_coverted:false,
        covert_name:String::from("tmp_converted"),
        // sink: Sink::try_new(&(OutputStream::try_default().unwrap().1)).unwrap()
    });
}

// const _STREAM: OnceLock<OutputStream> = OnceLock::new();
// const STREAM_HANDLE: OnceLock<OutputStreamHandle> = OnceLock::new();
// const SINK: OnceLock<Sink> = OnceLock::new();
pub fn init_processing(){

    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    // _STREAM.get_or_init(|| {
    //         _stream
    // });

    // STREAM_HANDLE.get_or_init(|| {
    //     stream_handle
    // });

    // SINK.get_or_init(||{
    //     Sink::try_new(&stream_handle).unwrap()
    // });
}

pub fn on_input(path: &str) -> &str {
    
    //check file exist and read
    let file_path: &Path = Path::new(path);
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
    cur_state.path = String::from(path);
    cur_state.is_playable = true;
    cur_state.is_playing = false;
    cur_state.is_coverted = false;

    file_name
} 

pub fn toggle_play_music() -> String {

    let mut cur_state = GLOBAL_STATE.lock().unwrap();
    
    match cur_state.is_playable {
        true => match  cur_state.is_playing {
            true => {
                cur_state.is_playing = false;
                "Stop".to_string()
            },
            false => {
                cur_state.is_playing = true;
                thread::spawn(||{
                    play_music();
                    // std::thread::sleep(std::time::Duration::from_secs(10));
                });
                
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
    cur_state.path = String::from("./");
}

fn get_file_path() -> String {
    let cur_state = GLOBAL_STATE.lock().unwrap();
    Path::new(&(cur_state.path)).to_str().unwrap().to_string()
}


fn play_music(){
    
    let (_stream, stream_handle) = match OutputStream::try_default() {
      Ok((v,t)) => (v,t),  
      Err(_) => {
        println!("can not access speaker");
        return
      }
    };
    let path = get_file_path();
    let file_path: &Path = Path::new(path.as_str());

    
    let file: BufReader<File> = BufReader::new(
        match File::open(file_path) {
            Ok(v) => v,
            Err(_) => {
                println!("can not read file");
                return
            }
        }
    );
    let source = match Decoder::new(file) {
        Ok(v) => v,
        Err(_) => {
            println!("can not create source");
            return
        }
    };

    let sink = match Sink::try_new(&stream_handle) {
        Ok(v) => v,
        Err(_) => {
            println!("can not create Sink");
            return
        }
    };
    sink.append(source);
    // cur_state.sink.append(source);
    std::thread::sleep(std::time::Duration::from_secs(50));

}