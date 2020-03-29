fn add(x: isize, y: isize) -> isize{
    x + y
}
fn main(){
    println!("{}", add(1, 2));
    
    //関数を変数に束縛できる
    let f: fn(isize, isize) -> isize = add;
    let a = f(2, 3);
    println!("{}", a);
}