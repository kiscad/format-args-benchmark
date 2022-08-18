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
    let rand_str10: String = (0..10)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let rand_isize: isize = rng.gen();
    let rand_i32: i32 = rng.gen();
    let rand_u64: u64 = rng.gen();
    let rand_u8: u8 = rng.gen();
    let rand_f64: f64 = rng.gen();
    let rand_f32: f32 = rng.gen();

    // literal str
    println!("Hello");
    // literal str as argument
    println!("Hello, {}!", "world");
    // String as argument
    println!("rand string: {}", rand_str10);
    println!("rand string: {rand_str10}!");
    println!("{argument}", argument = rand_str10);
    // literal int as argument
    println!("The number is {}", 1);
    // int value as argument
    println!("rand int is {}", rand_isize);
    println!("{value}", value = rand_u8);
    // tuple as argument
    println!("{:?}", (rand_f32, rand_i32));
    println!("{:?}", (rand_u64, &rand_str10));
    // multi-arguments
    println!("{} {}", rand_f32, rand_u64);
    println!("{1} {} {0} {}", rand_u8, rand_str10);
    println!("{a} {c} {b}", a = rand_str10, b = rand_i32, c = rand_f64);
    println!("{s} {}", rand_u64, s = rand_str10);
    // debug format
    println!("{:#?}", (rand_f32, &rand_str10));
    println!("{:?}", (rand_f32, &rand_str10));
    // sign/#/0
    println!("{:04}", rand_u8);
    println!("Hello {:+}!", rand_i32);
    println!("{:#x}!", rand_isize);
    println!("Hello {:05}!", rand_u8);
    println!("Hello {:05}!", -(rand_u8 as i16));
    println!("{:#010x}!", rand_u8);
    // Width
    println!("Hello {:5}!", rand_str3);
    println!("Hello {:1$}!", rand_str3, 5);
    println!("Hello {1:0$}!", 5, rand_str3);
    println!("Hello {:width$}!", rand_str3, width = 5);
    let width = 5;
    println!("Hello {:width$}!", rand_str3);
    // Fill/Alignment
    println!("Hello {:<5}!", rand_str3);
    println!("Hello {:-<5}!", rand_str3);
    println!("Hello {:^5}!", rand_str3);
    println!("Hello {:>5}!", rand_str3);
    println!("Hello {:^15}!", format!("{:?}", Some(rand_str3.clone())));
    // Precision
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
