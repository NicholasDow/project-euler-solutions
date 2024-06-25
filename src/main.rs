mod solution1_multiple3and5;
fn main() {
    // let (sol, indexes) = solution1_multiple3and5::total();
    // println!("{}", sol);
    // debug formatting, debug trait!!!!
    // println!("{:?}", indexes);

    // let dc = solution1_multiple3and5::double_counted();
    // println!("{}", dc);

    // println!("the actual solution {}", sol-dc);

    let ns = solution1_multiple3and5::new_approach();
    dbg!(ns);
}
