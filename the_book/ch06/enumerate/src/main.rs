enum IpAddrKind {
    V4,
    V6,
}

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

#[derive(Debug)]
enum Message {
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    Color(i32, i32, i32),
}

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
    // let quit_message = Message::Quit;
    // quit_message.handle();
    Message::Quit.handle();
    // let move_message = Message::Move{x:2,y:3};
    // move_message.handle();
    Message::Move{x:2,y:3}.handle();
    // let write_message = Message::Write(String::from("hello"));
    // write_message.handle();
    Message::Write(String::from("hello")).handle();
    // let color_message = Message::Color(0,0,0);
    // color_message.handle();
    Message::Color(0,0,0).handle();
}

fn main() {
    all_routes();
    all_messages_1();
    all_messages_2();
}
