use std::thread;

use global_stack::GlobalStack;

mod global_stack {
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    pub struct GlobalStack<T> {
        stack: Arc<Mutex<Vec<T>>>,
    }

    impl<T> Clone for GlobalStack<T> {
        fn clone(&self) -> Self {
            Self {
                stack: self.stack.clone(),
            }
        }
    }

    impl<T> Default for GlobalStack<T> {
        fn default() -> Self {
            Self {
                stack: Arc::default(),
            }
        }
    }

    impl<T> GlobalStack<T> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn push(&self, value: T) {
            self.stack.lock().unwrap().push(value);
        }

        pub fn pop(&self) -> Option<T> {
            self.stack.lock().unwrap().pop()
        }

        pub fn len(&self) -> usize {
            self.stack.lock().unwrap().len()
        }
    }
}

fn main() {
    let stack = GlobalStack::new();

    stack.push(42);

    let th = thread::spawn({
        let stack = stack.clone();
        move || {
            stack.push(42);
        }
    });

    let _ = stack.pop().unwrap();

    th.join().unwrap();

    assert_eq!(stack.len(), 1);
}
