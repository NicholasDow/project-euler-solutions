pub fn palprime() -> u64{
    // this is a candidate for this but it's slow
    // pub fn is_pal(n:u64) {
    //     let mut length = (n as f32).log10().floor() as u32;
    //     let mut val = n;
        
    //     for _ in 0..(length as f32/2).floor() {
    //         let small_end = val % 10;
    //         let big_end = val % pow(10,length);
    //         if ( small_end != big_end) {
    //             return false;
    //         }
    //         length -= 1;
    //         val = 
    //     }
    //     return true;
    // }

    /**
     * this solution is just going to turn the number into a list
     * it's not as fast but there are going to be fewer corner cases
     * 
     * 
     * I think the issue with this code is that rust requires so much casting to deal with numbers of this size
     */
    pub fn is_pal(n:u64) -> bool{
        // need to review rust semantics to figure out if this is ok
        let mut value = n;
        let mut values = Vec::new();
        while value != 0{
            values.push(value%10);
            value = ((value as f64) / 10.0).floor() as u64 ;
        }
        let mid: u64 = (values.len() as f32/2.0).floor() as u64;
        for i in 0..mid {
            if values.get(i as usize) != values.get((values.len()-1-i as usize) as usize) {
                return false;
            }
        }
        true
    }
    let mut largest:u64 = 0;
    for i in 1..=999 {
        for j in 1..=999 {
            let candidate = (i as u64)*(j as u64);
            if is_pal(candidate) {
                largest = largest.max(candidate)
            }
        }
    }
    largest
}