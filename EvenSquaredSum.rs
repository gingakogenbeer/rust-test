fn square_sum(n: isize) -> isize{   //  isize型を引数として、返り値isize型で定義
    (0..n)                          //  レンジリテラル 0 - 9をループ
    .filter(|i| i % 2 == 0)         //  高階関数 + クロージャ
    .map(|i| i * i)
    .sum()
}
fn main(){
    println!("{}", square_sum(10));
}