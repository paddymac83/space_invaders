use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use rusty_audio::Audio;

use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};

use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    player::Player,
    render,
};

fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    // add audio files to audio struct - name and then path
    audio.add("explode", ".wav");
    audio.add("lose", ".wav");
    audio.add("move", ".wav");
    audio.add("pew", ".wav");
    audio.add("startup", ".wav");
    audio.add("win", ".wav");

    // play the tune
    audio.play("startup");

    // the audio is played in a separate thread in parallel
    // we'd run off main and the threads would terminate
    // waits till all the audio stops playing before running off main

    // access the terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // render loop in a separte thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        render_screen(render_rx);
    });

    // add gameloop
    let mut player = Player::new();  // new instance of Player struct
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        // per frame init
        let delta = instant.elapsed();
        instant = Instant::now();   // provides a delta value based on the loop elapsed time
        let mut curr_frame = new_frame();

        // Input handlers for the game
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                    }
                    _ => {}
                }
            }
        }
        // draw and render section
        player.update(delta);    // pass loop elapsed time into the from_millis(50) and erode it..  
        if invaders.update(delta) {  // if invaders move then play move sound
            audio.play("move"); 
        }

        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];  // anything that implements the Drawable trait
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        let _ = render_tx.send(curr_frame);  // ignore errors as child thread starts up
        thread::sleep(Duration::from_millis(1)); // add 1ms delay to allow render to catch up

        // win/lose
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }

        if invaders.reached_bottom() {
        audio.play("lose");
        break 'gameloop;
        }
    }
  

    // Cleanup
    drop(render_tx); // drop trans end
    render_handle.join().unwrap();
    audio.wait();

    // undo all the stdout 
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())

}
