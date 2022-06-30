use std::{any::Any, marker::PhantomData};

use rand::prelude::SliceRandom;

trait HasFact {
    fn name() -> &'static str;
    fn fact() -> &'static str;
}

impl<T> HasFact for Vec<T> {
    fn name() -> &'static str {
        "Vec"
    }

    fn fact() -> &'static str {
        [
            "Vec is heap-allocated",
            "Vec may re-allocate on growing",
            "Vec<T> is an owned version of &[T]",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
    }
}

impl HasFact for [i32] {
    fn name() -> &'static str {
        "[i32]"
    }

    fn fact() -> &'static str {
        [
            "[i32] is a Dynamically Sized Type",
            "[i32] is a view into memory",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
    }
}

impl HasFact for dyn Any {
    fn name() -> &'static str {
        "dyn Any"
    }

    fn fact() -> &'static str {
        [
            "dyn Any doesn't have size known at compile time",
            "dyn Any can be any type!",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
    }
}

impl HasFact for () {
    fn name() -> &'static str {
        "()"
    }

    fn fact() -> &'static str {
        [
            "() is zero bytes in size",
            "() is returned by functions without return type specified",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
    }
}

struct Fact<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized + HasFact> Fact<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    pub fn fact(&self) -> String {
        format!("Fact about {}: {}.", T::name(), T::fact())
    }
}

fn main() {
    let fact: Fact<Vec<()>> = Fact::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());

    let fact: Fact<[i32]> = Fact::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());

    let fact: Fact<dyn Any> = Fact::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());

    let fact: Fact<()> = Fact::new();
    println!("{}", fact.fact());
    println!("{}", fact.fact());
    println!("{}", fact.fact());
}
