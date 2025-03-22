#![feature(portable_simd)]
#[macro_use] extern crate time_test;

mod tests;
mod encoding;
mod encoding_simd;

fn main() {
    println!("This program does nothing! Run the test suite (`cargo test`) to execute the challenges.");
}
