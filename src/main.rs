// Result<T, E> enum is used for error handling
// enum Result<T,E> {
//   Ok(T),
//   Err(E),
// }
//
// T and E are generic type params.
// Just like the Option enum, the Result enum is also brought into
// scope by the prelude.

use std::fs::File;

#[allow(unused_variables)]
fn main() {
    let f = File::open("hello.txt").unwrap();
}
