fn the_vector() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    v.push(100);
    v.push(32);
    v.push(57);
    v
}

fn indexed() {
    let v = the_vector();

    let third: i32 = v[2];
    debug_assert_eq!(third, 57);

    let third: &i32 = &v[2];
    debug_assert_eq!(*third, 57);

    match v.get(2) {
        Some(&elem) => debug_assert_eq!(elem, 57),
        None => (),
    }

    match v.get(3) {
        Some(_) => (),
        None => debug_assert_eq!(v.len(), 3),
    }
}

fn ref_for() {
    let v = the_vector();

    let mut sum = 0;
    for val in &v {
        sum += val;
    }
    debug_assert_eq!(sum, 189);
    debug_assert_eq!(v, [100,32,57]);
}

fn mov_for() {
    let v = the_vector();

    let mut sum = 0;
    for val in v {
        sum += val;
    }
    debug_assert_eq!(sum, 189);
    // `v` moved due to this implicit call to `.into_iter()`
    // help: consider borrowing to avoid moving into the for loop: `&v`
    // hence `v` cannot be accessed here
    //      debug_assert_eq!(v, [100,32,57]);
}

fn mut_for() {
    let mut v = the_vector();

    for val in &mut v {
        *val += 50;
    }

    let mut sum = 0;
    for val in &v {
        sum += val;
    }
    debug_assert_eq!(sum, 339);
}

fn main() {
    indexed();
    ref_for();
    mov_for();
    mut_for();
}
