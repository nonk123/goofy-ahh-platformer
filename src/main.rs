use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
    },
    ExecutableCommand,
};

mod game;
mod runner;
mod util;

fn main() -> crossterm::Result<()> {
    {
        let mut stdout = std::io::stdout().lock();

        let kb_flags = KeyboardEnhancementFlags::REPORT_EVENT_TYPES;

        stdout
            .execute(PushKeyboardEnhancementFlags(kb_flags))?
            .execute(EnableMouseCapture)?;
    }

    use crossterm::terminal;

    terminal::enable_raw_mode()?;

    let result = runner::crossterm_runner();
    terminal::disable_raw_mode()?;

    result?;

    {
        let mut stdout = std::io::stdout().lock();

        stdout
            .execute(DisableMouseCapture)?
            .execute(PopKeyboardEnhancementFlags)?;
    }

    Ok(())
}
