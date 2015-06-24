
//! Tools for dealing with universes (this is a doc comment, and is shown on
//! the crate index page. The ! makes it apply to the parent of the comment,
//! rather than what follows)

/// The Overloadable Trait
trait Overloadable {
    /// one method only
    fn answer(&self) -> i32;
}

/// Default Type. The value is 0.
struct Default;
/// OneArg Type. Contains tuple with one value.
struct OneArg(i32);
/// TwoArg Type. Contains tuple with two values.
struct TwoArgs(i32, i32);

impl Overloadable for Default {
    /// Returns zero
    fn answer(&self) -> i32 { 0 }
}
impl Overloadable for OneArg {
    /// Returns value
    fn answer(&self) -> i32 { self.0 }
}
impl Overloadable for TwoArgs {
    /// Return the sum of two items of the tuple
    fn answer(&self) -> i32 { self.0 + self.1 }
}


/// fn calculate<T: Overloadable> (arg: T) -> i32 
/// executes overloaded function for each passed Overloadable Item
/// ```
///    println!("{}", calculate(Default)); 	// 0
///    println!("{}", calculate(OneArg(1)));	// 1
///    println!("{}", calculate(TwoArgs(2, 3)));// 5
/// ```
fn calculate<T: Overloadable> (arg: T) -> i32 {
    arg.answer()
}


/// Main function
fn main() {
    println!("{}", calculate(Default));
    println!("{}", calculate(OneArg(1)));
    println!("{}", calculate(TwoArgs(2, 3)));
}
