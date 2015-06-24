//! This is Overloadable module

/// The FunArgs trait contains only <B>exec</B> function
pub trait FunArgs {
    /// executes calculation for the passed item
    fn exec(&self) -> i32;
}

/// Default Type.
pub struct Default;
/// OneArg Type. Contains tuple with one value.
pub struct OneArg(i32);
/// TwoArg Type. Contains tuple with two values.
pub struct TwoArgs(i32, i32);

impl FunArgs for Default {
    ///
    /// Returns Zero
    ///
    fn exec(&self) -> i32 { 0 }
}
impl FunArgs for OneArg {
    ///
    /// Returns the contained item
    ///
    fn exec(&self) -> i32 { self.0 }
}
impl FunArgs for TwoArgs {
    ///
    /// Returns the sum of the contained item
    ///
    fn exec(&self) -> i32 { self.0 + self.1 }
}

/// fn calculate < T: FunArgs > (arg: T) -> i32 
///
/// executes overloaded function for each passed Item with FunArgs trait.
/// Returns i32.
///
/// # Examples
///
/// ```
///    println!("{}", calculate(Default));          // 0
///    println!("{}", calculate(OneArg(1)));        // 1
///    println!("{}", calculate(TwoArgs(2, 3)));    // 5
/// ```
pub fn calculate<T: FunArgs> (arg: T) -> i32 {
    arg.exec()
}


#[cfg(test)]
pub mod test_calculate {
    #[test]
    pub fn default()
    {        
        assert_eq!(super::calculate(super::Default), 0);
    }
    #[test]
    pub fn one_arg()
    {
        assert_eq!(super::calculate(super::OneArg(1)), 1);
    }
    #[test]
    pub fn two_args()
    {
        assert_eq!(super::calculate(super::TwoArgs(2,3)), 5);
    }
}


/// The Main function
pub fn main() {
    println!("{}", calculate(Default));
    println!("{}", calculate(OneArg(1)));
    println!("{}", calculate(TwoArgs(2, 3)));
}
