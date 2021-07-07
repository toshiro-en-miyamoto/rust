# Packages, Crates, and Modules

A crate is a binary or library. A package must contain zero or one library crate, and no more. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

```sh
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

- Here, we have a package that only contains `src/main.rs`, meaning it only contains a binary crate named `my-project`.
- If a package contains `src/main.rs` and `src/lib.rs`, it has two crates: a library and a binary, both with the same name as the package.
- A package can have multiple binary crates by placing files in the `src/bin` directory: each file will be a separate binary crate.

Cargo follows a convention that:
- `src/main.rs` is the crate root of a binary crate with the same name as the package.
- if the package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package, and `src/lib.rs` is its crate root.

`src/main.rs` and `src/lib.rs` are called **crate roots**. The reason for their name is that the contents of either of these two files

- form a module named `crate` at the root of the crate’s module structure,

The module structure is known as the *module tree*.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

Notice that the entire module tree is rooted under the implicit module named `crate`.

In the crate root file `src/lib.rs`, you can declare the `front_of_house` module whose body will be in `src/front_of_house.rs`.

```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

Then you define the `front_of_house` module in `src/front_of_house.rs`.

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
