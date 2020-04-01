struct Celsius(f64);
struct Kelvin(f64);

//impl+型名で、型名の対する実装
impl Celsius{
    //第一引数が self, &mut self, &dself, Box<self> の場合はメソッド
    fn to_kelvin(self) -> Kelvin {
        Kelvin(self.0 + 273.15)
    }
    fn from_kelvin(k: Kelvin) -> Self {
        Celsius(k.0 - 273.15)
    }
}
fn main(){
    let abs_zero = Kelvin(0.0);
    let tri_point = Celsius(0.0);

    let celsius = Celsius::from_kelvin(abs_zero);
    let kelvin = tri_point.to_kelvin();

    }