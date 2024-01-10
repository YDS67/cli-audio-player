use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::fs;
use rodio::Source; 

pub fn playback(rx: &Receiver<(bool, bool)>) {
    let current_dir = ".";

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

    ncurses::setlocale(ncurses::LcCategory::all, "");
    let screen = crate::Screen(ncurses::initscr());
    ncurses::scrollok(screen.0, true);

    let mut play = true;
    let mut skip = false;

    let mut play_request = rx.try_recv();


    loop {
        for entry in fs::read_dir(current_dir).unwrap() {
            let path = entry.unwrap().path();
            let pstr = path.into_os_string().into_string().unwrap();
            let file = std::fs::File::open(pstr.clone()).unwrap();
            let res = rodio::Decoder::new(BufReader::new(file));
            let sink = rodio::Sink::try_new(&handle).unwrap();
            match res {
                Ok(buff) => {
                    let buffc = buff.buffered();
                    sink.append(buffc);
                    ncurses::clear();
                    ncurses::wprintw(screen.0, &format!("Space to pause/play, S to skip, E to exit.\n"));
                    ncurses::wprintw(screen.0, &format!("Now playing {}\n", pstr));
                    ncurses::wrefresh(screen.0);
    
                    while !sink.empty() {
    
                        match play_request {
                            Ok(play_result) => {
                                play = play_result.0;
                                skip = play_result.1;
                
                            },
                            Err(_) => {
                            },
                        };
                        if play {
                            sink.play();
                            if skip {
                                sink.skip_one();
                                skip = false
                            }
                        } else {
                            sink.pause();
                        }
                        std::thread::sleep(std::time::Duration::from_secs_f64(crate::FT_DESIRED));
                
                        play_request = rx.try_recv();
                
                    }
                },
                Err(_) => {}
            }
        }
    }
    
}