#![crate_type = "dylib"]
#[no_mangle]
pub extern "C" fn add(lhv : i32, rhv :i32) -> i32 {
    lhv + rhv
}

