use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let rand_str10: String = (0..10)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    // literal str
    println!("Hello");
    // literal str as argument
    println!("Hello, {}!", "world");
    // String as argument
    println!("rand string: {}", rand_str10);
    println!("rand string: {rand_str10}!");
    println!("{argument}", argument = rand_str10);
}
