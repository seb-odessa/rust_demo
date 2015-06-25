use std::fmt::Debug;

fn print<T : Debug>(value : T) {
    println!("value = {:?}", value);
}

#[derive(Debug)]
struct Boxed<T> {
    payload : T
}

fn main() {
    print(1);
    print(2.5);
    print("Helo World");

    print(Boxed{payload:1});
    print(Boxed{payload:(2.5)});
    print(Boxed{payload:"Hello world"});
}

