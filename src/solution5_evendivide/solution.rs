use std::collections::HashSet;

pub fn evendivide() -> u32 {

}

fn factors(n:u32) -> HashSet{
    let mut counts = HashSet::new();
    let upperLimit = f64::sqrt(n) as u64 + 1; 
    for i in 1..upperLimit {
        if n %i == 0 {
            //unfinished
        }
    }
}