pub struct Keyboard {
    keys: [bool; 16]
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: [false; 16]
        }
    }

    pub fn press(&mut self, key: usize) {
        self.keys[key] = true;
    }

    pub fn release(&mut self, key: usize) {
        self.keys[key] = false;
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }
}