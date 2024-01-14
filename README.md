# Simple audio player for terminal

Tested on Ubuntu. *Doesn't work on Windows* (because ncurses), need to use another ui library.

Used rodio crate for audio, terminal_input and ncurses for ui.

**Installation**: download the binary. You can rename it any way you like and copy it to your bin directory.

**Usage**: when launched plays all music files in current directory (mp3, ogg, wav and flac are recognized by extension, will try to decode other files, then skip them if they can't be decoded). 

Loops over the whole file list (the order is system-dependent, so you can consider it random). UI shows directory name, file name and file format. You can pause or skip forward, but not back. I hope to implement more controls soon.

**Building**: `cargo build --release` should work. You'll need `libasound-dev` on Ubuntu/Debian, for other possible dependencies check out rodio and ncurses documentation.
