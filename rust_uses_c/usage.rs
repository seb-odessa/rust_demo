#[link(name="clang_add", kind="dylib")]
extern {
    fn add(lhv : i32, rhv : i32) -> i32;
}

fn main() {
    let x = unsafe { add(2,42) };
    println!("add(2,42) = {}", x);
}
