use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::fs;
use rodio::Source; 

pub fn playback(state_other: Arc<Mutex<crate::State>>) {
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
                },
                Err(_) => {}
            }

            loop {
                let mut s_other = state_other.lock().unwrap();
                s_other.file_num = counter;
                s_other.file_name = pstr.clone();

                if s_other.play {
                    sink.play();
                    if s_other.skip {
                        sink.skip_one();
                        s_other.skip = false;
                    }
                } else {
                    sink.pause();
                }
                drop(s_other);

                std::thread::sleep(std::time::Duration::from_secs_f64(crate::FT_DESIRED));

                if sink.empty() {
                    break
                }
        
            }
        }
    }
    
}