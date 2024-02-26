/* run all functions in lib.rs */

use trust::add;
use trust::sub;
use trust::mul;
use trust::div;

fn main() {
    println!("Hello, world!");
    println!("add(1, 2) = {}", add(1, 2));
    println!("sub(1, 2) = {}", sub(1, 2));
    println!("mul(1, 2) = {}", mul(1, 2));
    println!("div(1, 2) = {}", div(1, 2));
}
