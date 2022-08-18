use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    let rand_isize: isize = rng.gen();
    let rand_u8: u8 = rng.gen();

    // literal int as argument
    println!("The number is {}", 1);
    // int value as argument
    println!("rand int is {}", rand_isize);
    println!("{value}", value = rand_u8);
}
