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
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.text") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
