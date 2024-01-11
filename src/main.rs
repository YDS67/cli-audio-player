use std::sync::{Arc, Mutex};
use std::thread;

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

pub const FT_DESIRED: f64 = 1.0 / 60.0;

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
    let state_player = Arc::clone(&state);
    let state_display = Arc::clone(&state);

    thread::spawn(|| audio::playback(state_player));

    ncurses::setlocale(ncurses::LcCategory::all, "");
    let screen = Screen(ncurses::initscr());
    ncurses::scrollok(screen.0, true);
    let stdin = std::io::stdin();
    let mut input_stream = unsafe { InputStream::init_with_ncurses(stdin.lock(), screen.0) };

    let mut listen = false;
    let mut prepare = false;

    loop {
        if listen {

            match input_stream.next_event().unwrap() {
                Event::KeyPress {
                    modifiers: Modifiers::NONE,
                    key: KeyInput::Codepoint('s'),
                    ..
                } => {
                    let mut s_main = state.lock().unwrap();
                    if s_main.play {
                        s_main.skip = true;
                        s_main.message = format!("track skipped\n")
                    } else {
                        s_main.message = format!("unpause to skip\n")
                    }
                    drop(s_main)
                }
                Event::KeyPress {
                    modifiers: Modifiers::NONE,
                    key: KeyInput::Codepoint(' '),
                    ..
                } => {
                    let mut s_main = state.lock().unwrap();
                    s_main.play = !s_main.play;
                    s_main.message = format!("paused: {}\n", !s_main.play);
                    drop(s_main)
                }
                Event::KeyPress {
                    modifiers: Modifiers::NONE,
                    key: KeyInput::Codepoint('e'),
                    ..
                } => {
                    return;
                }
                _ => {}
            }

            listen = false;
            prepare = false
        } else {
            if prepare {
                listen = true
            } else {
                prepare = true
            }
        }

        let s_display = state_display.lock().unwrap();

        ncurses::clear();
        ncurses::wprintw(
            screen.0,
            &format!("Space to pause/play, S to skip, E to exit.\n"),
        );
        ncurses::wprintw(
            screen.0,
            &format!(
                "Now playing track [{}] {}\n",
                s_display.file_num, s_display.file_name
            ),
        );
        ncurses::wprintw(screen.0, &s_display.message);
        ncurses::wrefresh(screen.0);

        drop(s_display);
        
        std::thread::sleep(std::time::Duration::from_secs_f64(FT_DESIRED));
    }
}

