use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let rand_str3: String = (0..3)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("Hello {:<5}!", rand_str3);
    println!("Hello {:-<5}!", rand_str3);
    println!("Hello {:^5}!", rand_str3);
    println!("Hello {:>5}!", rand_str3);
    println!("Hello {:^15}!", format!("{:?}", Some(rand_str3)));
}
