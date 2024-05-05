pub struct FibonacciIterator {
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
        if self.n1 == 0 {
            return None;
        }
        if self.n0 > self.n1 {
            self.n1 = 0;
            return Some(self.n0);
        }
        let f = self.n0;
        let n2 = self.n0.wrapping_add(self.n1);
        self.n0 = self.n1;
        self.n1 = n2;
        return Some(f);
    }
}
