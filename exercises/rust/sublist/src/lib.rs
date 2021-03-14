#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_same_list<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    for i in 0..first_list.len() {
        if first_list[i] != second_list[i] {
            return false;
        }
    }
    return true;
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.len() == second_list.len() {
        if is_same_list(first_list, second_list) {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    } else if first_list.len() < second_list.len() {
        let check_len = first_list.len();
        let max_pos = second_list.len() - first_list.len() + 1;
        for  i in 0..max_pos+0 {
            let check_list= &second_list[i..(i+check_len)];
            if is_same_list(first_list, check_list) {
                return Comparison::Sublist;
            }
        }
        return Comparison::Unequal;

    } else {
        let check_len = second_list.len();
        let max_pos = first_list.len() - second_list.len();
        for  i in 0..max_pos {
            let check_list = &second_list[i..(i+check_len)];
            if is_same_list(first_list, check_list) {
                return Comparison::Sublist;
            }
        }
        return Comparison::Unequal;
    }
}
