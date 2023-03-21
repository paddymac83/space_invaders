use crate::frame::{Drawable, Frame};
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false, // as this is just starting
            timer: Timer::from_millis(50),   // this is the speed of location update
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta); // update timer with elapsed time
        if self.timer.ready && !self.exploding {   // self.timer.ready is time up, move the shot
            if self.y > 0 {
                self.y -=1 ;
            }
            self.timer.reset();   // start next time loop
        }
    }
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(250);  // to allow enough time to see explosion
    }
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)  // if time out + explore OR hit top of screen
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|" };   // explosion or a laser!
    }
}