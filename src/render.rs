use crate::frame::Frame;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

// opt by only changing from last frame
pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        // if we want to drae whole frame, queue a bunc and flush
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap(); // set to blue or error if not conn to terminal
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            // if the char has changed
            if *s != last_frame[x][y] || force {
                // move to location
                stdout.queue(MoveTo(x as u16, y as u16));
                print!("{}", *s);  // print without a line a deref str, which is the new str
            }

        }
    }
    stdout.flush().unwrap();  // execute the queued commands!

}