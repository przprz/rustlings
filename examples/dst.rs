/// Run with:
/// ```sh
/// cargo run --example dst
/// ```

use std::fmt::Debug;

// Fill in the blanks to make it work.
fn print_array<E: Debug, const N: usize>(arr: [E; N]) {
    println!("{:?}", arr);
}

fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
