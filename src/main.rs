use std::io;
use std::io::Read;
fn main() {
    for b in io::stdin().bytes() {
        print!("{}", b.unwrap() as char);
    }
}
