fn f() -> bool {
   false
}
///```
///assert!(f());
///```

#[test]
fn f_should_return_true() {
    assert!(f());
}

fn main() {
    println!("f() -> {:?}", f());
}

