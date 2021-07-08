# The `Option` enum

*Null* is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

The problem isn’t really with the concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`, and it is defined by the standard library as follows:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. In addition, so are its variants: you can use `Some` and `None` directly without the `Option::` prefix. The `Option<T>` enum is still just a regular enum, and `Some(T)` and `None` are still variants of type `Option<T>`.

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

When we have a `Some` value, we know that a value is present and the value is held within the `Some`. When we have a `None` value, in some sense, it means the same thing as null: we don’t have a valid value.

```rust
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
    assert_eq!(hard_add(five, absent_number), None);
    assert_eq!(soft_add(five, absent_number), Some(5));
    assert_eq!(hard_add(absent_number, Some(7)), None);
    assert_eq!(soft_add(absent_number, Some(7)), Some(7));
    assert_eq!(hard_add(five, Some(7)), Some(12));
    assert_eq!(soft_add(five, Some(7)), Some(12));
    assert_eq!(hard_add(hard_add(five, Some(7)), Some(3)), Some(15));
    assert_eq!(soft_add(soft_add(five, Some(7)), Some(3)), Some(15));
}
```
