use super::text_art::TextArt;

#[derive(Clone)]
pub struct Animation {
    frames: Vec<TextArt>,
    current_frame: usize,
}

impl Animation {
    pub fn new(frames: Vec<TextArt>) -> Self {
        Self {
            frames,
            current_frame: 0,
        }
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
    }

    pub fn next_frame(&mut self) -> &TextArt {
        let frame = &self.frames[self.current_frame];

        if self.current_frame + 1 >= self.frames.len() {
            self.current_frame = 0;
        } else {
            self.current_frame += 1;
        }

        frame
    }
}
