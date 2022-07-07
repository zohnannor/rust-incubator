macro_rules! btreemap {
    ($($key:expr => $value:expr),* $(,)?) => {
        std::collections::BTreeMap::from([
            $( ($key, $value) ),*
        ])
    };
}

pub(crate) use btreemap; // https://stackoverflow.com/a/31749071/11294165
