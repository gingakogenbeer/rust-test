//traitの定義
trait DuckLike{
    fn quack(&self);
    fn walk(&self){
        println!("walking");
    }
}
//Unit構造体で型名だけ定義した
struct Duck;
//impl + trait for 型名で定義できる
impl DuckLike for Duck{
    //traitで実装されていないメソッドを実装側で定義できる
    fn quack(&self){
        println!("quack");
    }
}
struct Tsuchinoko;
impl DuckLike for Tsuchinoko{
    fn quack(&self){
        println!("mew");
    }
    fn walk(&self){
        println!("wriggling");
    }
}
impl DuckLike for i64{
    fn quack(&self){
        for _ in 0..*self{
            println!("wow");
        }
    }
}
//トレイト境界
//DuckLikeトレイトを実装する型D定義し、引数duckの型として、トレイトのメソッドを実行
fn duck_go<D: DuckLike>(duck: D){
    duck.quack();
    duck.walk();
}
fn main(){
    let duck = Duck;
    let tsuchinoko = Tsuchinoko;
    let i = 3;
    duck.quack();
    tsuchinoko.quack();
    tsuchinoko.walk();
    i.quack();

    duck_go(duck);
}