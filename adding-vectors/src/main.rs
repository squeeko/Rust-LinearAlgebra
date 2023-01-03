/*
Adding vectors - place the vectors as the tail from one vector connects to the head of the other vector
*/
fn main() {
    // add two vectors
    let a = vec![10, 20, 30];
    let b = vec![40, 50, 60];
    let c: Vec<i32> = a.iter().zip(b).map(|(a, b)| a + b).collect();

    println!("Vec a + b = {:?}", c);
}
