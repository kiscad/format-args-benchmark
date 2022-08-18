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

    println!("Hello {:5}!", rand_str3);
    println!("Hello {:1$}!", rand_str3, 5);
    println!("Hello {1:0$}!", 5, rand_str3);
    println!("Hello {:width$}!", rand_str3, width = 5);
    let width = 5;
    println!("Hello {:width$}!", rand_str3);
}
