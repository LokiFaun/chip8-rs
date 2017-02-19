const STACK_SIZE: usize = 16;

pub trait IStack {
    fn push(&mut self, reg: u16);
    fn pop(&mut self) -> u16;
    fn get(&self) -> u16;
    fn get_pointer(&self) -> u16;
    fn get_stack(&self) -> &[u16];
}

pub struct Stack {
    stack: [u16; STACK_SIZE],
    stack_pointer: u16,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
        }
    }
}

impl IStack for Stack {
    fn push(&mut self, reg: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = reg;
    }

    fn pop(&mut self) -> u16 {
        let ret_val = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
        ret_val
    }

    fn get(&self) -> u16 {
        self.stack[self.stack_pointer as usize]
    }

    fn get_pointer(&self) -> u16 {
        self.stack_pointer
    }

    fn get_stack(&self) -> &[u16] {
        &self.stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_push() {
        let mut stack = Stack::new();
        assert_eq!(stack.get_pointer(), 0);
        stack.push(0x1234);
        assert_eq!(stack.get_pointer(), 1);
        assert_eq!(stack.get(), 0x1234);
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::new();
        assert_eq!(stack.get_pointer(), 0);
        stack.push(0x1234);
        assert_eq!(stack.get_pointer(), 1);
        assert_eq!(stack.pop(), 0x1234);
        assert_eq!(stack.get_pointer(), 0);
    }
}