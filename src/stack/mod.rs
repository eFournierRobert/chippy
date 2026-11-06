/// A stack of subroutine return addresses.
pub struct Stack {
    memory: Vec<u16>,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    /// Pushes a return address onto the stack.
    pub fn push(&mut self, value: u16) {
        self.memory.push(value);
    }

    /// Removes and returns the top address, or `None` if empty.
    pub fn pop(&mut self) -> Option<u16> {
        self.memory.pop()
    }
}
