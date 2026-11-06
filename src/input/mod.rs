use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io;

/// The pressed state of the 16 CHIP-8 keys.
pub struct Keyboard {
    keys: [bool; 16],
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Keyboard {
    /// Creates a keyboard with no keys pressed.
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    /// Marks a key as pressed.
    ///
    /// # Panics
    ///
    /// Panics if `key` is outside `0..16`.
    pub fn press(&mut self, key: usize) {
        self.keys[key] = true;
    }

    /// Marks a key as released.
    ///
    /// # Panics
    ///
    /// Panics if `key` is outside `0..16`.
    pub fn release(&mut self, key: usize) {
        self.keys[key] = false;
    }

    /// Returns whether a key is pressed.
    ///
    /// # Panics
    ///
    /// Panics if `key` is outside `0..16`.
    pub fn is_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }

    /// Returns the lowest-numbered pressed key, if any.
    pub fn is_key_pressed(&self) -> Option<u8> {
        for (key, pressed) in self.keys.iter().enumerate() {
            if *pressed {
                return Some(key as u8);
            }
        }

        None
    }

    /// Blocks until a mapped key event or Ctrl-C is received.
    ///
    /// Returns `None` for Ctrl-C and propagates terminal input errors.
    pub fn poll_keyboard() -> io::Result<Option<(usize, KeyEventKind)>> {
        loop {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('c' | 'C'))
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.kind == KeyEventKind::Press
                {
                    return Ok(None);
                }
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    continue;
                }
                if let Some(chip8_key) = Self::chip8_key(key.code) {
                    return Ok(Some((chip8_key, key.kind)));
                }
            }
        }
    }

    fn chip8_key(code: KeyCode) -> Option<usize> {
        match code {
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                '1' => Some(0x1),
                '2' => Some(0x2),
                '3' => Some(0x3),
                '4' => Some(0xC),
                'q' => Some(0x4),
                'w' => Some(0x5),
                'e' => Some(0x6),
                'r' => Some(0xD),
                'a' => Some(0x7),
                's' => Some(0x8),
                'd' => Some(0x9),
                'f' => Some(0xE),
                'z' => Some(0xA),
                'x' => Some(0x0),
                'c' => Some(0xB),
                'v' => Some(0xF),
                _ => None,
            },
            _ => None,
        }
    }
}
