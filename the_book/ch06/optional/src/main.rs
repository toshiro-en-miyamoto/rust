fn hard_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match a {
        None => None,
        Some(i) => match b {
            None => None,
            Some(j) => Some(i + j),
        },
    }
}

fn soft_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match a {
        None => match b {
            None => None,
            Some(_) => b,
        },
        Some(i) => match b {
            None => a,
            Some(j) => Some(i + j),
        },
    }
}

fn main() {
    let five: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;
    debug_assert_eq!(hard_add(five, absent_number), None);
    debug_assert_eq!(soft_add(five, absent_number), Some(5));
    debug_assert_eq!(hard_add(absent_number, Some(7)), None);
    debug_assert_eq!(soft_add(absent_number, Some(7)), Some(7));
    debug_assert_eq!(hard_add(five, Some(7)), Some(12));
    debug_assert_eq!(soft_add(five, Some(7)), Some(12));
    debug_assert_eq!(hard_add(hard_add(five, Some(7)), Some(3)), Some(15));
    debug_assert_eq!(soft_add(soft_add(five, Some(7)), Some(3)), Some(15));
}
