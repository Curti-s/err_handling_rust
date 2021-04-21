// Result<T, E> enum is used for error handling
// enum Result<T,E> {
//   Ok(T),
//   Err(E),
// }
//
// T and E are generic type params.
// Just like the Option enum, the Result enum is also brought into
// scope by the prelude.

// when panic! macro is used, your program will print the error message,
// unwind and clean up the stack then quit. It mostly occurs
// when a bug of some kind has been deteced and it's not clear to the
// programmer how to handle the error
//
// Unwinding the stack entails, Rust walking up the stack and cleaning
// up the data from each function it encounters; which is a ton of work.
//
// Alternatively, one can immediately abort w/o cleaning. This can be
// achieved by adding `panic = 'abort'` to the appropriate [profile] section
// w/i cargo.toml file.
//      [profile.release[
//      panic = 'abort'

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
