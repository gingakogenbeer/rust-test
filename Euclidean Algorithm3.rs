use std::io::Write; //writeトレイト 整形したに文字列をストリームへ書きだす
use std::str::FromStr;  //文字列を解析し型変換
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
    let mut numbers = Vec::new();   //u64

    for arg in std::env::args().skip(1){    //skip(1)でプログラム名をSkip
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument")); //コマンドライン引数をu64型にパースしnumbersに追加
    }
    if numbers.len() == 0{  //入力チェック
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();  //unwrap()はエラーメッセージの出力が成功したかチェック
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..]{     //vec2番目以降の要素への参照をmに借用する
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}