use std::io::BufReader;
use std::sync::mpsc::Receiver;
use std::fs;
use rodio::Source;

//rx.recv_timeout(std::time::Duration::from_secs_f64(FT_DESIRED))

pub fn playback(rx: &Receiver<(bool, bool)>) {
    let current_dir = ".";

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    ncurses::setlocale(ncurses::LcCategory::all, "");
    let screen = crate::Screen(ncurses::initscr());
    ncurses::scrollok(screen.0, true);

    for entry in fs::read_dir(current_dir).unwrap() {
        let path = entry.unwrap().path();
        let pstr = path.into_os_string().into_string().unwrap();
        //println!("Sussessfully found file: {}", pstr);
        let file = std::fs::File::open(pstr.clone()).unwrap();
        let res = rodio::Decoder::new(BufReader::new(file));
        match res {
            Ok(buff) => {
                let buffc = buff.buffered();
                sink.append(buffc);
                ncurses::wprintw(screen.0, &format!("Sussessfully read file: {}\n", pstr));
                ncurses::wrefresh(screen.0);
            },
            Err(_) => {}
        }
    }

    ncurses::wprintw(screen.0, &format!("\n"));
    ncurses::wprintw(screen.0, &format!("Space to pause/play, S to skip, E to exit.\n"));
    ncurses::wrefresh(screen.0);
    ncurses::wprintw(screen.0, &format!("\n"));
    ncurses::wrefresh(screen.0);
    ncurses::delscreen(screen.0);


    let mut play = true;
    let mut skip = false;
    //let mut iter: i32 = 0;

    let mut play_request = rx.try_recv();

    loop {
        //iter += 1;
        match play_request {
            Ok(play_result) => {
                play = play_result.0;
                skip = play_result.1;
                //println!("Request sent at loop {}", iter)
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
    
}