
pub fn into_tuple<T, I>(mut split: I) -> (T, T)
where
    I: Iterator<Item = T> {

    let one = split.next().expect("Iterator has one element");
    let two = split.next().expect("Iterator has two elements");
    (one, two)
}
