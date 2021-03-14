#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list, second_list) {
        (l, r) if l.len() == r.len() => {
            if l == r {
                return Comparison::Equal;
            } else {
                return Comparison::Unequal;
            }
        },
        (l, r) if l.len() < r.len() => {
            let check_len = l.len();
            let max_pos = r.len() - l.len() + 1;
            for  i in 0..max_pos {
                let check_list= &r[i..(i+check_len)];
                if l == check_list {
                    return Comparison::Sublist;
                }
            }
            return Comparison::Unequal;
    
        },
        (l, r) if l.len() > r.len() => {
            let check_len = r.len();
            let max_pos = l.len() - r.len() + 1;
            for  i in 0..max_pos {
                let check_list = &l[i..(i+check_len)];
                if r == check_list {
                    return Comparison::Superlist;
                }
            }
            return Comparison::Unequal;
        },
        _ => return Comparison::Unequal,
    }
}
