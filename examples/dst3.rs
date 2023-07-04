// 🌟🌟🌟 Sometimes we want to limit the size of a variable,
// e.g when using in embedding environments,
// then const expressions will fit your needs.

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
    where
        Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    // ...
}

// Fix the errors in main.
fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 40]); // Size of &str ?
    // check_size([(); 40].map(|_| "".to_string()));  // Size of String?
    check_size(['中'; 8]); // Size of char ?
    let s= [(); 40].map(|_| "".to_string());  // Size of String?
    println!("{:?}", s);

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
