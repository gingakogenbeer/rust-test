use std::io;
fn gcd(mut x:u64, mut y:u64) -> u64{
    assert!(x != 0 && y != 0);  //assert!マクロ, どちらの引数も0ではないことをチェック
    while y != 0{
        if y < x {
            let t = y;
            y = x;
            x = t;
        }
        y = y % x;
    }
    x
}
fn main(){
    println!("Please input two numbers.");
    let mut num1 = String::new();
    let mut num2 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let mut num1: u64 = num1.trim().parse().expect("Please type a number");
    let mut num2: u64 = num2.trim().parse().expect("Please type a number");
    println!("{}", gcd(num1, num2));
}