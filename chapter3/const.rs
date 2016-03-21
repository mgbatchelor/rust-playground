static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 42;

fn big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    println!("Lang: {}", LANGUAGE);

    println!("Thres: {}", THRESHOLD);

    println!("big? {}", big(100));


    // THRESHOLD = 1000;
}
