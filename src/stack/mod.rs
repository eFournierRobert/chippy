pub struct stack {
    memory: Vec<u16>
}

impl stack {
    pub fn push(&mut self, value: u16) {
        self.memory.push(value);
    }

    pub fn pop(&mut self) -> Option<u16> {
        self.memory.pop()
    }
}