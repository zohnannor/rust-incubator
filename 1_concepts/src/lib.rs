#![allow(clippy::missing_panics_doc, clippy::must_use_candidate, dead_code)]

use std::sync::Arc;

use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

#[derive(Debug, Default)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

#[derive(Debug, Default)]
struct Node<T> {
    data: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            data,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T, const N: usize> From<[T; N]> for List<T> {
    fn from(arr: [T; N]) -> Self {
        let mut list = Self::new();
        for el in arr {
            list.push_back(el);
        }
        list
    }
}

impl<T> List<T> {
    /// Returns the length of this [`List<T>`].
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the [`List<T>`] is empty.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends new node to the front of this [`List<T>`].
    pub fn push_front(&mut self, data: T) {
        let new_head = Node::new(data);

        match self.head.take() {
            // List had a head, make new_head and old_head point to each other
            Some(old_head) => {
                old_head.lock().prev = Some(new_head.clone());
                new_head.lock().next = Some(old_head);
                self.head = Some(new_head);
            }
            // List is empty, make tail and head point to new node.
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }

        self.len += 1;
    }

    /// Appends new node to the back of this [`List<T>`].
    pub fn push_back(&mut self, data: T) {
        let new_tail = Node::new(data);

        match self.tail.take() {
            // List had a tail, make new_tail and old_tail point to each other
            Some(old_tail) => {
                old_tail.lock().next = Some(new_tail.clone());
                new_tail.lock().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            // List is empty, make head and tail point to new node.
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }

        self.len += 1;
    }

    /// Removes node from the front of this [`List<T>`].
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            // List had a head, need to fix new head

            // Adjust the old head's successor to become the new head
            match old_head.lock().next.take() {
                // Old head pointed to something, remove it's predecessor
                Some(new_head) => {
                    new_head.lock().prev.take();
                    self.head = Some(new_head);
                }
                // Old head was the only element, fix the tail
                None => {
                    self.tail.take();
                }
            }

            self.len -= 1;

            Arc::try_unwrap(old_head).ok().unwrap().into_inner().data
        })
    }

    /// Removes node from the back of this [`List<T>`].
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            // List had a tail, need to fix new tail

            // Adjust the old tail's predecessor to become the new tail
            match old_tail.lock().prev.take() {
                // Old tail pointed to something, remove it's successor
                Some(new_tail) => {
                    new_tail.lock().next.take();
                    self.tail = Some(new_tail);
                }
                // Old tail was the only element, fix the head
                None => {
                    self.head.take();
                }
            }

            self.len -= 1;

            Arc::try_unwrap(old_tail).ok().unwrap().into_inner().data
        })
    }

    /// Returns the reference to the front of this [`List<T>`].
    pub fn front(&self) -> Option<MappedMutexGuard<T>> {
        self.head
            .as_ref()
            .map(|head| MutexGuard::map(head.lock(), |head| &mut head.data))
    }

    /// Returns the reference to the back of this [`List<T>`].
    pub fn back(&self) -> Option<MappedMutexGuard<T>> {
        self.tail
            .as_ref()
            .map(|tail| MutexGuard::map(tail.lock(), |tail| &mut tail.data))
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len;
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;

    use super::*;

    #[test]
    fn create() {
        let list = List::<()>::new();
        assert_eq!(list.len, 0);
    }

    #[test]
    fn push_front() {
        let mut list = List::new();
        assert_eq!(list.len, 0);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len, 3);
    }

    #[test]
    fn pop_front() {
        let mut list = List::new();
        assert_eq!(list.len, 0);
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len, 2);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len, 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len, 0);
    }

    #[test]
    fn push_back() {
        let mut list = List::new();
        assert_eq!(list.len, 0);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len, 3);
    }

    #[test]
    fn pop_back() {
        let mut list = List::new();
        assert_eq!(list.len, 0);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len, 2);

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.len, 1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len, 0);
    }

    #[test]
    fn front() {
        let mut list = List::new();
        assert_eq!(list.len, 0);
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len, 2);

        assert_eq!(list.front().as_deref(), Some(&2));

        *list.front().unwrap().deref_mut() *= 2;
        assert_eq!(list.front().as_deref(), Some(&4));
    }

    #[test]
    fn back() {
        let mut list = List::new();
        assert_eq!(list.len, 0);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len, 2);

        assert_eq!(list.back().as_deref(), Some(&2));

        *list.back().unwrap().deref_mut() *= 2;
        assert_eq!(list.back().as_deref(), Some(&4));
    }

    #[test]
    fn iter_loop() {
        let mut list = List::new();
        assert_eq!(list.len, 0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len, 3);

        for _ in list {}
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        assert_eq!(list.len, 0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.len, 3);

        let it = list.into_iter();
        assert_eq!(it.len(), 3);
        let mut it = it.rev();
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn from() {
        let list = List::from([1, 2, 3, 4]);
        let mut list = list.into_iter();
        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), Some(2));
        assert_eq!(list.next(), Some(3));
        assert_eq!(list.next(), Some(4));
        assert_eq!(list.next(), None);
    }
}
