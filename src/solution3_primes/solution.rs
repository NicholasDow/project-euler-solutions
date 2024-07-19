// This didn't work because the largest number that divides our number might not be prime
// pub fn primes(n:u32) -> Option<u32>{
//     for i in (0..(n as f32).sqrt().floor() as u32).rev() {
//         if n%i == 0 {
//             return Some(i);
//         }
//     }
//     None
// }

pub fn primes(n:u64) -> u64{
    fn helper(i:u64, candidate:u64) -> (u64, u64){
        for j in 2..((i as f64).sqrt().floor() as u64) {
            if i%j == 0 {
                return helper(i/j, candidate.max(j))
            }
        }
        (i, candidate)
    }
    let (v, _c) = helper(n, 1);
    v 
}