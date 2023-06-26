use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use super::Game;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    // TODO: implement accessor methods instead of making the API public.
    pub code: KeyCode,
    pub ctrl: bool,
}

impl Game {
    pub fn is_key_held(&self, code: KeyCode, ctrl: bool) -> bool {
        self.held_keys.contains(&Key { code, ctrl })
    }

    pub fn process_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => {
                if let KeyCode::Char('c') = key_event.code {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        self.exit_requested = true;
                    }
                }

                let key = Key {
                    code: key_event.code,
                    ctrl: key_event.modifiers.contains(KeyModifiers::CONTROL),
                };

                match key_event.kind {
                    KeyEventKind::Press => {
                        self.held_keys.insert(key);
                    }
                    KeyEventKind::Release => {
                        self.held_keys.remove(&key);
                    }
                    KeyEventKind::Repeat => (),
                }
            }
            _ => (),
        }
    }

    pub fn process_player_input(&mut self) {
        let controls = self
            .get_player()
            .map(|player| player.movement_controls)
            .map(|mut controls| {
                controls.left = self.is_key_held(KeyCode::Char('a'), false);
                controls.right = self.is_key_held(KeyCode::Char('d'), false);
                controls.up = self.is_key_held(KeyCode::Char('w'), false);
                controls.down = self.is_key_held(KeyCode::Char('s'), false);
                controls.jump = self.is_key_held(KeyCode::Char(' '), false);
                controls
            });

        if let Some(controls) = controls {
            self.get_player_mut()
                .map(|player| player.movement_controls = controls);
        }
    }
}
