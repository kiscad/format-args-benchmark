use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let rand_isize: isize = rng.gen();
    let rand_i32: i32 = rng.gen();
    let rand_u8: u8 = rng.gen();

    println!("{:04}", rand_u8);
    println!("Hello {:+}!", rand_i32);
    println!("{:#x}!", rand_isize);
    println!("Hello {:05}!", rand_u8);
    println!("Hello {:05}!", -(rand_u8 as i16));
    println!("{:#010x}!", rand_u8);
}
