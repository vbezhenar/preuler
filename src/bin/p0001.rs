fn main() {
    let answer = (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<u32>();
    println!("{answer}");
}
