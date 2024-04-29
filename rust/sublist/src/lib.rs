#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if is_equal(a, b) {
        return Comparison::Equal;
    }

    if is_sublist(a, b) {
        return Comparison::Sublist;
    }

    if is_superlist(a, b) {
        return Comparison::Superlist;
    }

    return Comparison::Unequal;
}

fn is_equal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    return a == b;
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.is_empty() { return true; }

    for w in b.windows(a.len()) {
        if w == a { return true; }
    }

    return false;
}

fn is_superlist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    return is_sublist(b, a);
}