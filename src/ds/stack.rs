#[derive(Debug)]
#[warn(dead_code)]
struct VecStack<T> {
    data: Vec<T>,
    len: usize,
}

impl<T> VecStack<T> {
    fn new() -> Self {
        VecStack {
            len: 0,
            data: Vec::new(),
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.data.pop()
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn size(&self) -> usize {
        self.len
    }

    fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }
        self.data.get(self.len - 1)
    }
}

#[cfg(test)]
mod test {

    use super::VecStack;
    #[test]
    fn basics() {
        let mut stack = VecStack::new();
        assert_eq!(stack.size(), 0);

        stack.push(1);
        assert_eq!(stack.size(), 1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop().unwrap(), 3);
        assert_eq!(*stack.peek().unwrap(), 2);
        assert_eq!(stack.size(), 2);
        stack.pop();
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }
}
