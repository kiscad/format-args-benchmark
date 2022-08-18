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
    let rand_f32: f32 = rng.gen();

    println!("Hello {0} is {1:.5}", rand_str3, rand_f32);
    println!("Hello {1} is {2:.0$}", 5, rand_str3, rand_f32);
    println!("Hello {0} is {2:.1$}", rand_str3, 5, rand_f32);
    println!("Hello {} is {:.*}", rand_str3, 5, rand_f32);
    println!("Hello {1} is {2:.*}", 5, rand_str3, rand_f32);
    println!("Hello {} is {2:.*}", rand_str3, 5, rand_f32);
    println!(
        "Hello {} is {number:.prec$}",
        rand_str3,
        prec = 5,
        number = rand_f32
    );
}
