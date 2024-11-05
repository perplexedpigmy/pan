#[macro_export]
/// Generates a limitless sequence, with the first element being `$first` and every consecutive
/// element is `$second`.
///
/// Example:
///
/// gen_seq_first_elem!(1, 0) // Generates the list 1,0,0,.....
macro_rules! gen_seq_first_elem {
  ($first:expr, $second:expr) => {
    std::iter::once($first).chain(std::iter::repeat($second))
  };
}

#[macro_export]
macro_rules! remove_element {
  ($vec:expr, $pattern:pat) => {
    $vec.retain(|e| !matches!(e, $pattern))
  };
}

#[macro_export]
macro_rules! replace_element {
  ($vec:expr, $pattern:pat, $replacement:expr) => {
    remove_element!($vec, $pattern);
    $vec.push($replacement);
  };
}
