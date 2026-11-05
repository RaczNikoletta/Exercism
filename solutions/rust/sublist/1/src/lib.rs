#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;

    let is_sublist = |a: &[T], b: &[T]| {
        if a.is_empty() {
            true
        } else {
            b.windows(a.len()).any(|window| window == a)
        }
    };

    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m == n && _first_list == _second_list => Equal,
        (m, n) if m < n && is_sublist(_first_list, _second_list) => Sublist,
        (m, n) if m > n && is_sublist(_second_list, _first_list) => Superlist,
        _ => Unequal,
    }
}