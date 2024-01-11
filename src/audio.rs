use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::fs;
use rodio::Source; 

pub fn playback(state_player: Arc<Mutex<crate::State>>) {
    let current_dir = ".";

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    loop {
        let mut counter = 0;
        for entry in fs::read_dir(current_dir).unwrap() {
            let path = entry.unwrap().path();
            let pstr = path.clone().into_os_string().into_string().unwrap();
            let file = std::fs::File::open(path).unwrap();
            let res = rodio::Decoder::new(BufReader::new(file));
            let sink = rodio::Sink::try_new(&handle).unwrap();

            match res {
                Ok(buff) => {
                    counter += 1;
                    let buffc = buff.buffered();
                    sink.append(buffc);
                    while !sink.empty() {
                        let mut s_player = state_player.lock().unwrap();
                        s_player.file_num = counter;
                        s_player.file_name = pstr.clone();
        
                        if s_player.play {
                            sink.play();
                            if s_player.skip {
                                s_player.skip = false;
                                sink.skip_one();
                                break;
                            }
                        } else {
                            sink.pause();
                        }
                        drop(s_player);
        
                        std::thread::sleep(std::time::Duration::from_secs_f64(crate::FT_DESIRED));
                    }
                },
                Err(_) => {}
            }
        }
    }
    
}