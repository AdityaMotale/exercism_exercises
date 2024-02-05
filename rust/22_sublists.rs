#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if is_sublist(_first_list, _second_list) {
        return Comparison::Superlist;
    }

    if is_sublist(_second_list, _first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}

pub fn is_sublist<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    if second.is_empty() {
        return true;
    }

    if first.len() < second.len() {
        return false;
    }

    for i in 0..=(first.len() - second.len()) {
        if first[i] == second[0] {
            let mut match_found = true;
            for j in 0..second.len() {
                if second[j] != first[i + j] {
                    match_found = false;
                    break;
                }
            }

            if match_found {
                return true;
            }
        }
    }

    false
}
