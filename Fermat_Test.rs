/*  フェルマーテスト    */
use std::io;
fn main(){
    println!("Please input number.");
    let mut p = String::new();
    io::stdin().read_line(&mut p).expect("Failed to read line");
    let mut p: u32 = p.trim().parse().expect("Please type a number");
    let mut a = 2;
    a = u32::pow(a, p-1) % p;
    println!("{}", a);
/*  Carmichael number(Under 100000)    */
    match p {
        561   | 1105  | 1729  | 
        2465  | 2821  | 6601  | 
        8911  | 10585 | 15841 | 
        29341 | 41041 | 46657 | 
        52633 | 62745 | 63973 | 
        75361 
           => println!("this is a Carmichael number"),
        _  => (),
    }
}
