use std::fmt;

struct Foo { payload : u32 }
impl Foo {
    fn new(value:u32) -> Foo {
        let foo = Foo{payload : value};
        println!("{} constuctor", foo);
        foo
    }
}
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{} destructor", self);
    }
}
impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "Foo({})",self.payload)
    }
}

fn main() {
    {        
        let _0 = Foo{payload : 0};
    }
    {
        let _1 = Foo::new(1);
        let _2 = Box::new(Foo::new(2));
    }
    let _3 = Foo::new(3);
}

