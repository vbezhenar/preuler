fn main() {
    let mut answer: Option<u32> = None;
    for i in 100..=999 {
        for j in i..=999 {
            let n = i * j;
            if is_palindrom(n) {
                if answer.is_none() || answer.is_some_and(|a| n > a) {
                    answer = Some(n)
                }
            }
        }
    }
    println!("{:?}", answer);
}

fn is_palindrom(n: u32) -> bool {
    let mut x = n;
    let mut y = 0;
    while x != 0 {
        y *= 10;
        y += x % 10;
        x /= 10;
    }
    y == n
}
