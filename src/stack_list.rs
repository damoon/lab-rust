use std::mem;

pub struct Stack<T> {
    head: Option<StackElement<T>>,
}

struct StackElement<T> {
    value: T,
    next: Box<Option<StackElement<T>>>,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Self {
        Stack::<T> {
            head: None,
        }
    }

    pub fn push(&mut self, v: T) {
        let next = mem::replace(&mut self.head, None);
        self.head = Some(StackElement::<T>{
            value: v,
            next: Box::new(next),
        });
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head {
            Some(_) => {
                let head = mem::replace(&mut self.head, None).unwrap();
                self.head = *head.next;
                Some(head.value)
            },
            None => None,
        }

    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn empty() {
        let mut stack = Stack::<&str>::new();
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn add_remove() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn add_keep_remove() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}