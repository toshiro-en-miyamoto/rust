# Strings

Rust has only one string type in the core language, which is the *string slice* `str` that is usually seen in its borrowed form `&str`, which are references to some UTF-8 encoded string data stored elsewhere.
String literals, for example, are stored in the program’s binary and are therefore string slices.

The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they usually mean the `String` and the string slice `&str` types, not just one of those types.
