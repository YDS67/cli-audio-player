use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

use terminal_input::{Event, InputStream, KeyInput, Modifiers};

pub struct Screen(ncurses::WINDOW);

impl Drop for Screen {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

pub const FT_DESIRED: f64 = 1.0/60.0;

mod audio;

fn main() {
    let mut play_music = true;
    let mut skip = false;
    let (tx, rx): (Sender<(bool, bool)>, Receiver<(bool, bool)>) = mpsc::channel();
    thread::spawn(move || {audio::playback(&rx)});

    ncurses::setlocale(ncurses::LcCategory::all, "");
    let screen = Screen(ncurses::initscr());
    ncurses::scrollok(screen.0, true);
    let stdin = std::io::stdin();
    let mut input_stream = unsafe { InputStream::init_with_ncurses(stdin.lock(), screen.0) };

    loop {
        let event = input_stream.next_event();

        match event.unwrap() {
            Event::KeyPress { modifiers: Modifiers::NONE, key: KeyInput::Codepoint(' '), .. } => {
                play_music = !play_music;
                tx.send((play_music, skip)).unwrap();
                ncurses::wprintw(screen.0, &format!("paused: {}\n", !play_music));
                ncurses::wrefresh(screen.0);
            }
            Event::KeyPress { modifiers: Modifiers::NONE, key: KeyInput::Codepoint('s'), .. } => {
                if play_music {
                    skip = true;
                    tx.send((play_music, skip)).unwrap();
                    skip = false;
                    ncurses::wprintw(screen.0, &format!("skipped a song\n"));
                    ncurses::wrefresh(screen.0);
                } else {
                    ncurses::wprintw(screen.0, &format!("unpause to skip\n"));
                    ncurses::wrefresh(screen.0);
                }
            }
            Event::KeyPress { modifiers: Modifiers::NONE, key: KeyInput::Codepoint('e'), .. } => {
               return;
            }
            _ => {}
        }

        std::thread::sleep(std::time::Duration::from_secs_f64(FT_DESIRED));
    }

    
    
}