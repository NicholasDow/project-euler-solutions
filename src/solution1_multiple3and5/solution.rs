use std::{collections::HashSet};

// I'm leaving my terrible attempts here: JFK I was stupid.
const LIM:u32 = 1000;
// pub fn total() -> (u32, Vec<i32>){
//     // this is the brute force solution since I am trying to learn rust

//     let mut sum:u32 = 0;
//     let nums = [3,5];
//     let mut indexes = Vec::new();
//     for num in nums.iter() {
//         let mut index = 0;
//         let mut current = 0;
//         // over counted num here
//         while current + num < LIM{
//             current += num;
//             sum += current;
//             index +=1;
//         }
//         indexes.push(index);
//     }

//     // we can of course solve with (n)(n+1)/2 *(3+5)
//     // lets check our answer using this method?
//     // I'm not going to bother because its harder to make sure I get exact solution. Will see if anyone took this approach in solutions
//     // it turns out that we are double counting
//     (sum, indexes)
// }

// pub fn double_counted() -> u32 {
//     let mut seen = HashSet::new(); 

//     fn helper(i1: u32, i2: u32, seen: &mut HashSet<(u32, u32)>) -> u32 {
//         let value = (3u32).pow(i1 as u32) * (5u32).pow(i2 as u32); // Calculate value
//         if value >= LIM {
//             return 0; 
//         }
//         if seen.contains(&(i1, i2)) {
//             return 0; 
//         }
//         println!("{value}");
//         seen.insert((i1, i2)); 
//         value + helper(i1 + 1, i2, seen) + helper(i1, i2 + 1, seen)
//     }

//     helper(1, 1, &mut seen) 
// }

#[allow(dead_code)]
pub fn new_approach() -> u32 {
    let mut seen = HashSet::new(); 

    fn helper(i1: u32, i2: u32, seen: &mut HashSet<u32>) -> u32 {
        
        let value:u32= if i1 != 0 && i2 != 0 {
            i1 * 3 * i2* 5
        } else if i1 == 0  {
            i2 *5
        }  else {
            i1*3
        }; 

        if value >= LIM {
            return 0; 
        }
        if seen.contains(&(value)) {
            return helper(i1+1, i2, seen) + helper(i1,i2+1, seen);
        }
        dbg!(value);
        dbg!(i1,i2);
        seen.insert(value); 
        value + helper(i1 + 1, i2, seen) + helper(i1, i2 + 1, seen)
    }

    helper(0,0, &mut seen) 
}
/*
I did the previous solution w/o looking up language specific details 
*/
#[allow(dead_code)]
pub fn better_solution() -> u32 {
    (0..1000).filter(|&x| x % 3 == 0|| x % 5 == 0).sum()
}
#[allow(dead_code)]
pub fn optimized() -> u32 {
    let gauss_sum = |n: u32| -> Box<dyn Fn(u32) -> u32> {
        Box::new(move |m| m * ((n - 1) / m) * (((n - 1) / m) + 1) / 2)
    };
    let gauss_limit_1000 = gauss_sum(1000);
    // the reason 15 work here is because every prime factorizaiton with both 3 and 5 has a multiple of 15
    gauss_limit_1000(3) + gauss_limit_1000(5) - gauss_limit_1000(15)
}