/*
Subtracting vectors - line up two vectors such that their tails are at the same coordinate
*/
fn main() {
    // add two vectors
    let a = vec![10, 20, 30];
    let b = vec![50, 80, 160];
    let c: Vec<i32> = a.iter().zip(b).map(|(a, b)| a - b).collect();

    println!("Vec a - b = {:?}", c);
}
