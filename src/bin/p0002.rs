fn main() {
    let mut f1 = 1;
    let mut f2 = 2;
    let mut sum = 2;
    while f2 <= 4_000_000 {
        f2 = f1 + f2;
        f1 = f2 - f1;
        if f2 % 2 == 0 {
            sum += f2;
        }
    }
    println!("{}", sum);
}
