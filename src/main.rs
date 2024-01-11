use std::thread;
use std::sync::{Arc, Mutex};

use terminal_input::{Event, InputStream, KeyInput, Modifiers};

pub struct Screen(ncurses::WINDOW);

impl Drop for Screen {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

#[derive(Debug, Clone)]
pub struct State {
    play: bool,
    skip: bool,
    file_num: usize,
    file_name: String,
    message: String,
}



pub const FT_DESIRED: f64 = 1.0/60.0;

mod audio;

fn main() {
    let state = State {
        play: true,
        skip: false,
        file_num: 0,
        file_name: format!(""),
        message: format!(""),
    };

    let state = Arc::new(Mutex::new(state));
    let state_other = Arc::clone(&state);

    thread::spawn(|| {audio::playback(state_other)});

    ncurses::setlocale(ncurses::LcCategory::all, "");
    let screen = Screen(ncurses::initscr());
    ncurses::scrollok(screen.0, true);
    let stdin = std::io::stdin();
    let mut input_stream = unsafe { InputStream::init_with_ncurses(stdin.lock(), screen.0) };

    loop {
        let event = input_stream.next_event();

        let mut s_main = state.lock().unwrap();

        match event.unwrap() {
            Event::KeyPress { modifiers: Modifiers::NONE, key: KeyInput::Codepoint('s'), .. } => {
                if s_main.play {
                    s_main.skip = true;
                    s_main.message = format!("track skipped\n")
                } else {
                    s_main.message = format!("unpause to skip\n")
                }
            }
            Event::KeyPress { modifiers: Modifiers::NONE, key: KeyInput::Codepoint(' '), .. } => {
                s_main.play = !s_main.play;
                s_main.message = format!("paused: {}\n", !s_main.play)
            }
            Event::KeyPress { modifiers: Modifiers::NONE, key: KeyInput::Codepoint('e'), .. } => {
               return;
            }
            _ => {}
        }

        ncurses::clear();
        ncurses::wprintw(screen.0, &format!("Space to pause/play, S to skip, E to exit.\n"));
        ncurses::wprintw(screen.0, &format!("Now playing track [{}] {}\n", s_main.file_num, s_main.file_name));
        ncurses::wprintw(screen.0, &s_main.message);
        ncurses::wrefresh(screen.0);

        drop(s_main);

        std::thread::sleep(std::time::Duration::from_secs_f64(FT_DESIRED));
    }

    
    
}