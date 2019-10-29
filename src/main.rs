
/**
 * By BingLi224
 * 20:11 THA 29/10/2019
 *
 * Rust 2018 module-in-subdirectory example.
 *
 * References:
 *  https://doc.rust-lang.org/stable/edition-guide/rust-2018/module-system/path-clarity.html
 *  https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#module-system-changes
 *  https://doc.rust-lang.org/rust-by-example/mod/split.html
 *  https://doc.rust-lang.org/reference/items/modules.html
 */

// declare the path 'subpath' as the subdirectory name and .rs file
mod subpath;

// import the structs in 'subpath/submod.rs' and 'subpath/submod2.rs'
use crate::subpath::submod::Int;
use crate::subpath::submod2::Float;

fn main() {
    let int = Int { val : 10 };
    let float = Float { val : 1.234 };

    println ! ( "int.val: {}", int.val );
    println ! ( "float.val: {}", float.val );
}
