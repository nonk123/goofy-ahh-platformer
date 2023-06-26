use std::time::{Duration, Instant};

use crossterm::event::{self};

pub use crossterm::event::Event as CrosstermEvent;

use crate::game::{player::new_player, renderer::Screen, Game};

pub const REFRESH_RATE: f32 = 60.0;
pub const REFRESH_DELAY_SECONDS: f32 = 1.0 / REFRESH_RATE;

pub fn crossterm_runner() -> crossterm::Result<()> {
    let mut game = Game::new();

    let player = new_player();
    game.spawn(player);

    let mut screen = Screen::new();

    let mut previous_frame = Instant::now();

    loop {
        while event::poll(Duration::ZERO)? {
            let event = event::read()?;
            game.process_event(event);
        }

        if game.exit_requested() {
            break;
        }

        screen.begin_frame();
        game.tick(&mut screen);
        screen.end_frame()?;

        if game.exit_requested() {
            break;
        }

        let current_frame = Instant::now();
        let dt = current_frame.duration_since(previous_frame).as_secs_f32();
        previous_frame = current_frame;

        if dt < REFRESH_DELAY_SECONDS {
            let diff = REFRESH_DELAY_SECONDS - dt;
            let diff = Duration::from_secs_f32(diff);
            std::thread::sleep(diff);
        }
    }

    Ok(())
}
