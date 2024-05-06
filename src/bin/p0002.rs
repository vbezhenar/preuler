fn main() {
    let fi = FibonacciIterator::new();
    let answer = fi
        .take_while(|f| *f <= 4_000_000)
        .filter(|f| f % 2 == 0)
        .sum::<u64>();
    println!("{answer}");
}

struct FibonacciIterator {
    n0: u64,
    n1: u64,
}

impl FibonacciIterator {
    pub fn new() -> FibonacciIterator {
        FibonacciIterator { n0: 0, n1: 1 }
    }
}

impl Iterator for FibonacciIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let f = self.n0;
        let n2_opt = self.n0.checked_add(self.n1);
        let n2 = n2_opt.expect("next value is too large");
        self.n0 = self.n1;
        self.n1 = n2;
        return Some(f);
    }
}
