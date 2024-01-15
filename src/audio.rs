use std::sync::mpsc::Receiver;
use std::fs;
use std::fs::File;
use rodio::{decoder::{Decoder, DecoderError}, Source};

pub fn playback(rx: &Receiver<(bool, bool)>) {
    let current_dir = std::env::current_dir().expect("Can't find current directory");
    let (_stream, handle) = rodio::OutputStream::try_default().expect("Can't open output stream (Rodio)");

    ncurses::setlocale(ncurses::LcCategory::all, "");
    let screen = crate::Screen(ncurses::initscr());
    ncurses::scrollok(screen.0, true);

    let mut play = true;
    let mut skip = false;

    let mut play_request = rx.try_recv();

    let sink = rodio::Sink::try_new(&handle).expect("Can't create Rodio Sink");


    loop {
        let mut counter = 0;
        let entries = fs::read_dir(current_dir.clone()).expect("ReadDir error");
        for entry in entries {
            let path = entry.expect("Reading entry path error").path();
            let pstr = format!("{}", current_dir.display());
            let dir_name = dir_name(&pstr);
            let pstr = format!("{}", path.display());
            let file_name = track_name(&pstr);
            let ext = track_format(&pstr);
            let ext_text = ext.display();
            let file_open = std::fs::File::open(path);
            match file_open {
                Ok(file) => {
                    let res = new_with_ext(file, ext);

                    match res {
                        Ok(buff) => {
                            counter += 1;
                            let buffc = buff.buffered();
                            sink.append(buffc);

                            ncurses::clear();
                            ncurses::wprintw(screen.0, &format!("[Space] to pause/resume, [S] to skip, [E] to exit.\n"));
                            ncurses::wprintw(screen.0, &format!("Current directory: {}\n", dir_name.clone()));
                            ncurses::wprintw(screen.0, &format!("Now playing track [{}]: {}\n", counter, file_name.clone()));
                            ncurses::wprintw(screen.0, &format!("Format: {}\n", ext_text.clone()));
                            ncurses::wrefresh(screen.0);
                            
                            while !sink.empty() {

                                match play_request {
                                    Ok(play_result) => {
                                        play = play_result.0;
                                        skip = play_result.1;
                                    },
                                    Err(_) => {}
                                }
                
                                if play {
                                    sink.play();
                                    if skip {
                                        skip = false;
                                        sink.skip_one();
                                    }
                                } else {
                                    sink.pause();
                                }
                
                                std::thread::sleep(std::time::Duration::from_secs_f64(crate::FT_DESIRED));

                                play_request = rx.try_recv();
                            }
                        },
                        Err(_) => {
                            ncurses::clear();
                            ncurses::wprintw(screen.0, &format!("[Space] to pause/resume, [S] to skip, [E] to exit.\n"));
                            ncurses::wprintw(screen.0, &format!("Current directory: {}\n", dir_name.clone()));
                            ncurses::wprintw(screen.0, &format!("No audio files could be found or decoded.\n"));
                            ncurses::wrefresh(screen.0);
                            //std::thread::sleep(std::time::Duration::from_secs_f64(crate::FT_DESIRED));
                        }
                    }
                }
                Err(_) => {
                    ncurses::clear();
                    ncurses::wprintw(screen.0, &format!("[Space] to pause/resume, [S] to skip, [E] to exit.\n"));
                    ncurses::wprintw(screen.0, &format!("Current directory: {}\n", dir_name.clone()));
                    ncurses::wprintw(screen.0, &format!("No audio files could be found or decoded.\n"));
                    ncurses::wrefresh(screen.0);
                    //std::thread::sleep(std::time::Duration::from_secs_f64(crate::FT_DESIRED));
                }
            }
        }
    }
    
}

fn track_name(text: &String) -> String {
    let text1: Vec<&str> = text.split(|c| c == '/' || c == '\\').collect();
    let letters: Vec<char> = text1[text1.len()-1].chars().collect();
    let mut new: String = String::new();
    for i in 0..letters.len() {
        new.push(letters[i]);
    }
    let split: Vec<&str> = new.split(|c| c == '.').collect();
    let mut res = String::new();
    for i in 0..(split.len()-1) {
        let chars: Vec<char> = split[i].chars().collect();
        for j in 0..chars.len() {
            res.push(cyr_to_lat(chars[j]))
        }
    }
    res
}

fn dir_name(text: &String) -> String {
    let text1: Vec<&str> = text.split(|c| c == '/' || c == '\\').collect();
    let letters: Vec<char> = text1[text1.len()-1].chars().collect();
    let mut new: String = String::new();
    for i in 0..(letters.len()).min(40) {
        new.push(cyr_to_lat(letters[i]));
    }
    new
}

fn track_format(text: &String) -> MusicFormat {
    let split: Vec<&str> = text.split(|c| c == '.').collect();
    let ext = split[split.len()-1];
    let answ: MusicFormat = match ext {
        "mp3" => MusicFormat::MP3,
        "wav" => MusicFormat::WAV,
        "ogg" => MusicFormat::OGG,
        "flac" => MusicFormat::FLAC,
        "MP3" => MusicFormat::MP3,
        "WAV" => MusicFormat::WAV,
        "OGG" => MusicFormat::OGG,
        "FLAC" => MusicFormat::FLAC,
        _ => MusicFormat::GEN,
    };
    answ
}

pub enum MusicFormat {
    MP3,
    WAV,
    OGG,
    FLAC,
    GEN,
}

impl MusicFormat {
    fn display(&self) -> String {
        match self {
            Self::MP3 => "mp3".to_string(),
            Self::WAV => "wav".to_string(),
            Self::OGG => "ogg".to_string(),
            Self::FLAC => "flac".to_string(),
            _ => "unknown".to_string(),
        }
    }
}

fn new_with_ext(file: File, ext: MusicFormat) -> Result<Decoder<File>, DecoderError> {
    let inner = match ext {
        MusicFormat::MP3 => Decoder::new_mp3(file),
        MusicFormat::WAV => Decoder::new_wav(file),
        MusicFormat::OGG => Decoder::new_vorbis(file),
        //MusicFormat::OGG => Decoder::new(file),
        MusicFormat::FLAC => Decoder::new_flac(file),
        _ => Decoder::new(file),
    }?;
    Ok(inner)
}

fn cyr_to_lat(letter: char) -> char {
    match letter {
        'а' => 'a',
        'б' => 'b',
        'в' => 'v',
        'г' => 'g',
        'д' => 'd',
        'е' => 'e',
        'ё' => 'e',
        'ж' => 'j',
        'з' => 'z',
        'и' => 'i',
        'й' => 'y',
        'к' => 'k',
        'л' => 'l',
        'м' => 'm',
        'н' => 'n',
        'о' => 'o',
        'п' => 'p',
        'р' => 'r',
        'с' => 's',
        'т' => 't',
        'у' => 'u',
        'ф' => 'f',
        'х' => 'h',
        'ц' => 'z',
        'ч' => 'c',
        'ш' => 's',
        'щ' => 's',
        'ъ' => '\'',
        'ы' => 'y',
        'ь' => '\'',
        'э' => 'e',
        'ю' => 'u',
        'я' => 'a',
        'А' => 'A',
        'Б' => 'B',
        'В' => 'V',
        'Г' => 'G',
        'Д' => 'D',
        'Е' => 'E',
        'Ё' => 'E',
        'Ж' => 'J',
        'З' => 'Z',
        'И' => 'I',
        'Й' => 'Y',
        'К' => 'K',
        'Л' => 'L',
        'М' => 'M',
        'Н' => 'N',
        'О' => 'O',
        'П' => 'P',
        'Р' => 'R',
        'С' => 'S',
        'Т' => 'T',
        'У' => 'U',
        'Ф' => 'F',
        'Х' => 'H',
        'Ц' => 'Z',
        'Ч' => 'C',
        'Ш' => 'S',
        'Щ' => 'S',
        'Ъ' => '\'',
        'Ы' => 'Y',
        'Ь' => '\'',
        'Э' => 'E',
        'Ю' => 'U',
        'Я' => 'A',
        _ => letter
    }
}