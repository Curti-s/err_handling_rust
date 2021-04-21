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
use std::io::ErrorKind;

#[allow(unused_variables)]
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}
