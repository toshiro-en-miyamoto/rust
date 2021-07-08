# Vectors

To create a new, empty vector, we can call the `Vec::new` function.
Then use the `push` method to add elements to it.

```rust
fn the_vector() -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    v.push(100);
    v.push(32);
    v.push(57);
    v
}
```

There are two ways to reference a value stored in a vector:
- using `&` and `[]`, which gives us a reference, or
- using the `get` method, which gives us an `Option<&T>`

```rust
fn indexed() {
    let v = the_vector();

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
```

`&v[3]` will cause the program to panic because it references a nonexistent element. `v.get(3)` returns `None` without panicking.

We can iterate through all of the elements rather than use indices to access one at a time.

```rust
fn ref_for() {
    let v = the_vector();

    let mut sum = 0;
    for val in &v {
        sum += val;
    }
    debug_assert_eq!(sum, 189);
    debug_assert_eq!(v, [100,32,57]);
}
```

Note that `for val in v` will cause the vector `v` to be moved into the for loop, hence `v` cannot be accessed after the loop.

```rust
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
```

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.

```rust
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
```
