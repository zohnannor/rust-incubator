#![allow(clippy::use_self)]

use std::{fmt, pin::Pin, rc::Rc};

trait MutMeSomehow {
    fn mut_me_somehow(self: Pin<&mut Self>);
}

impl<T: Unpin + Default> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = Pin::into_inner(self);
        core::mem::take(this.as_mut());
    }
}

impl<T: Unpin + Default> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = Pin::into_inner(self);
        if let Some(this) = Rc::get_mut(this) {
            core::mem::take(this);
        }
    }
}

impl<T: Unpin + Default> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = Pin::into_inner(self);
        this.clear();
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = Pin::into_inner(self);
        this.clear();
    }
}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(self: Pin<&mut Self>) {
        let this = Pin::into_inner(self);
        *this = [].as_ref();
    }
}

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self);
    }
}

impl<T: fmt::Debug> SayHi for Box<T> {}

impl<T: fmt::Debug> SayHi for Rc<T> {}

impl<T: fmt::Debug> SayHi for Vec<T> {}

impl SayHi for String {}

impl SayHi for &[u8] {}

fn main() {
    let mut b = Box::new(42);
    {
        let p = Pin::new(&mut b);
        Pin::as_ref(&p).say_hi();
        p.mut_me_somehow();
    }
    dbg!(b);

    let mut rc = Rc::new(42);
    {
        let p = Pin::new(&mut rc);
        Pin::as_ref(&p).say_hi();
        p.mut_me_somehow();
    }
    dbg!(rc);

    let mut v = vec![42, 42, 42];
    {
        let p = Pin::new(&mut v);
        Pin::as_ref(&p).say_hi();
        p.mut_me_somehow();
    }
    dbg!(v);

    let mut s = "42".to_string();
    {
        let p = Pin::new(&mut s);
        Pin::as_ref(&p).say_hi();
        p.mut_me_somehow();
    }
    dbg!(s);

    let mut sl = &[42, 42, 42][..];
    {
        let p = Pin::new(&mut sl);
        Pin::as_ref(&p).say_hi();
        p.mut_me_somehow();
    }
    dbg!(sl);
}
