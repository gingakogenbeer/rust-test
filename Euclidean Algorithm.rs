/*   Euclidean Algorithm   */
use std::io;
fn main() {
    println!("Please input two numbers.");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut x;
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    io::stdin().read_line(&mut num2).expect("Failed to read line");

    let mut num1: i32 = num1.trim().parse().expect("Please type a number");
    let mut num2: i32 = num2.trim().parse().expect("Please type a number");

    if num1 < num2 {
        num2 = num1 - num2;
        num1 -= num2;
        num2 += num1;
    }
    x = num1 % num2;
    while x != 0 {
        num1 = num2;
        num2 = x;
        x = num1 % num2;
    }
    if x == 0 { println!("greatest common divisor = {}", num2); }
    else { println!("greatest common divisor is nothing"); }
}
