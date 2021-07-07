# `enum`s

With `enum`s, we can enumerate *all possible variants*.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

We can then, for instance, define a function that takes any `IpAddrKind`:

```rust
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("V4"),
        IpAddrKind::V6 => println!("V6"),
    }
}

fn all_routes() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}
```

The `enum` below has a wide variety of types embedded in its variants:

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    Color(i32, i32, i32),
}
```

This `enum` has four variants with different types:

- `Quit` has no data associated with it at all.
- `Move` includes an anonymous `struct` inside it.
- `Write` includes a single `String`.
- `Color` includes three `i32` values.

The following `struct`s could hold the same data that the preceding `enum` variants hold:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different `struct`s, which each have their own type, we couldn’t as easily define a function to take any of these kinds of message as with the `Message enum`, which is a single type:

```rust
fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit!"),
        Message::Move{x,y} => println!("Move({}, {})", x, y),
        Message::Write(str) => println!("Write(\"{}\")", str),
        Message::Color(r,g,b) => println!("Color{:?}", (r,g,b)),
    }
}

fn all_messages_1() {
    handle_message(Message::Quit);
    handle_message(Message::Move{x:2,y:3});
    handle_message(Message::Write(String::from("hello")));
    handle_message(Message::Color(0,0,0));
}
```

We’re able to define methods on `enum`s as with `struct`s:

```rust
impl Message {
    fn handle(&self) {
        match self {
            Message::Quit => println!("{:?}", self),
            Message::Move{..} => println!("{:?}", self),
            Message::Write(_) => println!("{:?}", self),
            Message::Color(..) => println!("{:?}", self),
        }
    }
}

fn all_messages_2() {
    Message::Quit.handle();
    Message::Move{x:2,y:3}.handle();
    Message::Write(String::from("hello")).handle();
    Message::Color(0,0,0).handle();
}
```
