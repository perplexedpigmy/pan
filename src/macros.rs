#[macro_export]
macro_rules! sequence {
    ($first:expr, $second:expr) => {
        std::iter::once($first).chain(std::iter::repeat($second))
    };
}

