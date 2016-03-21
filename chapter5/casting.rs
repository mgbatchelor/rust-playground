fn main() {
    let decimal = 65.123123f32;

    println!("D: {}", decimal);

    let integer = decimal as u8;

    println!("I: {}", integer);

    let character = integer as char;
    println!("C: {}", character);

    println!("66000 as a u16 is: {}", 66000 as u16);

    println!("1000 as a u8 is : {}", 1000 as u8);

    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is : {}", 1000 % 256);
    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);

    println!("1000 as a i8 is : {}", 1000 as i8);
    println!(" 232 as a i8 is : {}", 232 as i8);

}
