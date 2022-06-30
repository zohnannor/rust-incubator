#![allow(clippy::missing_panics_doc)]

use std::sync::{Arc, Mutex};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

struct Node<T> {
    data: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Returns the length of this [`List<T>`].
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the [`List<T>`] is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends new node to the front of this [`List<T>`].
    pub fn push_front(&self, data: T) {
        let new_head = Node::new(data);

        match self.head.take() {
            Some(old_head) => todo!(),
            None => todo!(),
        }
    }

    /// Appends new node to the back of this [`List<T>`].
    pub fn push_back(&self, data: T) {
        todo!()
    }

    /// Removes node from the front of this [`List<T>`].
    pub fn pop_front(&self) -> Option<T> {
        todo!()
    }

    /// Removes node from the back of this [`List<T>`].
    pub fn pop_back(&self) -> Option<T> {
        todo!()
    }

    /// Returns the reference to the front of this [`List<T>`].
    pub fn front(&self) -> Option<&T> {
        todo!()
    }

    /// Returns the mutable reference to the front of this [`List<T>`].
    pub fn front_mut(&self) -> Option<&mut T> {
        todo!()
    }

    /// Returns the reference to the back of this [`List<T>`].
    pub fn back(&self) -> Option<&T> {
        todo!()
    }

    /// Returns the mutable reference to the back of this [`List<T>`].
    pub fn back_mut(&self) -> Option<&mut T> {
        todo!()
    }

    /// Returns the iterator over the references of elements of this [`List<T>`].
    pub fn iter<'a>(&self) -> Iter<'a, T> {
        todo!()
    }

    /// Returns the iterator over the mutable references of elements of this [`List<T>`].
    pub fn iter_mut<'a>(&self) -> IterMut<'a, T> {
        todo!()
    }
}

impl<'a, T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct Iter<'a, T>(&'a List<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct IterMut<'a, T>(&'a mut List<T>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
