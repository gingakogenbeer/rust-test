/*  フェルマーテスト    */
use std::io;
fn main(){
    println!("Please input number.");
    let mut p = String::new();
    io::stdin().read_line(&mut p).expect("Failed to read line");
    let mut p: u32 = p.trim().parse().expect("Please type a number");
    let mut a = 2;
    a = (a ^ (p - 1)) % p;
    println!("{}", a);
/*  擬素数判定(100000以下)    */
    if p == 561 || p == 1105 || p == 1729 ||
       p == 2465 || p == 2821 || p == 6601 ||
       p == 8911 || p == 10585 || p == 15841 || 
       p == 29341 || p == 41041 || p == 46657 || 
       p == 52633 || p == 62745 || p == 63973 || p == 75361 {
        println!("擬素数");
    }
}
