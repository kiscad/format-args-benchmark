use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let rand_f32: f32 = rng.gen();
    let rand_str10: String = (0..10)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{:#?}", (rand_f32, &rand_str10));
    println!("{:?}", (rand_f32, &rand_str10));
}
