use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;  // this is an alias

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" "); // push empty space
        }
        cols.push(col);
    }
    cols // return cols
}
// one complete new frame

// impl trait to draw into frame. Takes in mutable frame and fills it in with drawing
pub trait Drawable {
    fn draw(&self, frame: &mut Frame) {

    }
}