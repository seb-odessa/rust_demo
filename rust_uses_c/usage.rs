use std::old_io;

fn main() {
    #[link(name="clang_add", kind="dylib")]
    extern {fn add(lhv : i32, rhv : i32) -> i32;}
    let x = unsafe { add(2,42) };
    println!("add(2,42) = {}", x);
}
