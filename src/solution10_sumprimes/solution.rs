pub fn sumprimes() {
    let largest = 2_000_000;
    let mut valid: Vec<bool> = vec![true; largest - 2];
    let mut atn: Vec<usize> = (2..largest).collect();

    for i in 0..atn.len() {
        if i % 10000 == 0 {
            println!("{}", i);
        }
        if valid[i] {
            let v = atn[i];
            for j in (i + 1)..(largest - 2) {
                if valid[j] && atn[j] % v == 0 {
                    atn[j] = 0;
                    valid[j] = false;
                }
            }
        }
    }

    let sum: usize = atn.iter().sum();
    println!("Sum of primes less than {}: {}", largest, sum);
}