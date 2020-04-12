fn sorted_string(s: &str) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    s.into_iter().collect::<String>()
}
fn main(){
    let word = std::env::args().nth(1).expect("usage: word");
    println!("{}", word);
    let sorted_word = sorted_string(&word);
    println!("{}", sorted_word);
}