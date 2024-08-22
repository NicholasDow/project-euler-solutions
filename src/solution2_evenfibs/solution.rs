#[allow(dead_code)]
pub fn fib_optimized(n:u32) -> u32 {
    let mut var1: u32 = 1;
    let mut var2: u32 = 1;
    let mut total: u32 = 0;
    let mut temp: u32;
    while var2 < n {
        if var2 % 2 == 0 {
            total += var2
        }
        temp = var1 + var2;
        var1 = var2;
        var2 = temp;
    }

    total
}