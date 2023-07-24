
use std::{fs, path::Path, fs::File};
use lazy_static::lazy_static;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle, Source};
use std::thread;
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};

struct GlobalState {
    song_name: String,
    path: String,
    is_playing:bool,
    is_playable:bool,
    is_coverted:bool,
    covert_name:String,
    // sink: Sink
}

struct AudioPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

lazy_static!{
    static ref GLOBAL_STATE: Mutex<GlobalState> = Mutex::new(GlobalState{
        song_name: String::from("tmp"),
        path: String::from("./"),
        is_playing:false,
        is_playable:false,
        is_coverted:false,
        covert_name:String::from("tmp_converted"),
    });
}

pub fn init_processing(){

}

pub fn on_input(path: &str) -> &str {
    
    let file_path: &Path = Path::new(path);
    let file: BufReader<File> = BufReader::new(
        match File::open(file_path) {
            Ok(v) => v,
            Err(_) => {
                println!("File is corrupted!");
                return "File is corrupted!"
            }
        }
    );

    match Decoder::new(file) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid music file");
            return "Invalid music file"
        }
    };
    
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
                let audio_player = init_audio_player().lock().unwrap();
                audio_player.sink.pause();
                "Stop".to_string()
            },
            false => {
                cur_state.is_playing = true;
                thread::spawn(||{
                    play_music();
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

fn init_audio_player() -> &'static Mutex<AudioPlayer> {
    static mut CONF: MaybeUninit<Mutex<AudioPlayer>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    let (_stream, _stream_handle) = OutputStream::try_default().unwrap();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(
            AudioPlayer {
                stream:_stream,
                stream_handle:_stream_handle.clone(),
                sink:Sink::try_new(&_stream_handle).unwrap()
            }
        ));
    });
    unsafe { &*CONF.as_ptr() }
}

fn get_sink()-> Sink {
    let audio_player = init_audio_player().lock().unwrap();
    let sink = Sink::try_new(&audio_player.stream_handle).unwrap();
    sink
}


fn play_music(){

    let audio_player = init_audio_player().lock().unwrap();
    // let sink: Sink = get_sink();

    if audio_player.sink.is_paused() {
        audio_player.sink.play();
        return
    }

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
    
    audio_player.sink.append(source);
}