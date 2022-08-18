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
    let rand_i32: i32 = rng.gen();
    let rand_u64: u64 = rng.gen();
    let rand_f32: f32 = rng.gen();

    println!("{:?}", (rand_f32, rand_i32));
    println!("{:?}", (rand_u64, &rand_str10));
}
