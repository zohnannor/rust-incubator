mod declarative;

fn main() {
    use std::collections::BTreeMap;
    {
        let _map = declarative::btreemap! {
            1 => "hello",
            2 => "world!"
        };

        let x = "asd";
        let _map = declarative::btreemap! {
            x => "trailing",
            x => "comma ->",
        };

        let _map: BTreeMap<String, Vec<usize>> = declarative::btreemap! {};

        let _map = declarative::btreemap! {
            1 => declarative::btreemap! {
                1 => "nested",
                2 => "nested"
            }
        };
    }

    {
        let _map = procedural::btreemap! {
            1 => "hello",
            2 => "world!"
        };

        let x = "asd";
        let _map = procedural::btreemap! {
            x => "trailing",
            x => "comma ->",
        };

        let _map: BTreeMap<String, Vec<usize>> = procedural::btreemap! {};

        // error: expected `,`
        // 1 => procedural::btreemap! {
        //                          ^

        // let _map = procedural::btreemap! {
        //     1 => procedural::btreemap! {
        //         1 => "nested",
        //         2 => "nested"
        //     }
        // };
    }
}
