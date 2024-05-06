fn main() {
    let n: usize = 600851475143;
    let sieve_size: usize = (n as f64).sqrt() as usize;
    let mut sieve: Vec<bool> = Vec::with_capacity(sieve_size + 1);
    sieve.resize(sieve_size + 1, true);

    let i_max = (sieve_size as f64).sqrt() as usize;
    for i in 2..=i_max {
        if !sieve[i] {
            continue;
        }
        let mut k = i + i;
        while k <= sieve_size {
            sieve[k] = false;
            k += i;
        }
    }

    let mut m: usize = n;
    let mut i: usize = 2;
    'outer: loop {
        let i_max = (m as f64).sqrt() as usize;
        println!("Processing {m} with {i}..{i_max}");
        while i <= i_max {
            if sieve[i] {
                println!("Trying {i}");
                if m % i == 0 {
                    println!("Found {i}");
                    m /= i;
                    continue 'outer;
                }
            }
            i += 1;
        }
        break;
    }
    println!("{m}");
}
