use std::io::stdout;
use crossterm::event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

/// Terminal guard to manage the terminal in raw mode.
pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> std::io::Result<Self> {
        enable_raw_mode()?;

        if let Err(error) = execute!(
            stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
            )
        ) {
            let _ = disable_raw_mode();
            return Err(error);
        }

        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), PopKeyboardEnhancementFlags);
        let _ = disable_raw_mode();
    }
}