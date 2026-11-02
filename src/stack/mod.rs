pub struct Stack {
    memory: Vec<u16>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    pub fn push(&mut self, value: u16) {
        self.memory.push(value);
    }

    pub fn pop(&mut self) -> Option<u16> {
        self.memory.pop()
    }
}
