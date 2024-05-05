#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    return match (is_equal(a, b), is_sublist(a, b), is_sublist(b, a)) {
        (true, ..) => Comparison::Equal,
        (_, true, _) => Comparison::Sublist,
        (.., true) => Comparison::Superlist,
        _ => Comparison::Unequal,
    };
}

fn is_equal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    return a == b;
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    return a.is_empty() || b.windows(a.len()).any(|w| is_equal(a, w));
}
