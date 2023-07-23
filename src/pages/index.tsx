import style from '@/styles/style.module.css'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'

enum LogLevel {
  no_log,
  debug,
  info,
  warn,
  error
}

let tmp = 4;

listen('tauri://file-drop', event => {
  let file_path = String(event.payload);
  let control = document.getElementById("control");
  control!.innerHTML = "";
  invoke<string>('on_drop', { path: file_path })
    .then((response) => {
      let button = document.getElementById("toggle");
      button!.innerHTML = response;
    })
})

const onPlaying = () => {
  invoke<string>('play_music')
    .then((response) => {
      let button = document.getElementById("control");
      button!.innerHTML = response;
    })
}


export default function Home() {
  return (
    <>
      <div className={style.playmusic}>
        <div id="music-animation" className={style.musicanimation}>
          <span className={`${style.bar} ${style.bar1}`}></span>
          <span className={`${style.bar} ${style.bar2}`}></span>
          <span className={`${style.bar} ${style.bar3}`}></span>
          <span className={`${style.bar} ${style.bar4}`}></span>
        </div>
        <div className={style.musictoggle}>
          <a onClick={onPlaying} id="toggle" >Waiting...</a>
        </div>
      </div>
      <div className={style.musictoggle}>
        <a onClick={onPlaying} id="control" ></a>
      </div>
    </>
  )
}
