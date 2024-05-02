use preuler::FibonacciIterator;

fn main() {
    let fi = FibonacciIterator::new();
    let answer = fi
        .take_while(|f| *f <= 4_000_000)
        .filter(|f| f % 2 == 0)
        .sum::<u64>();
    println!("{}", answer);
}
