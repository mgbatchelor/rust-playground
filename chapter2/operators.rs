fn main() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("true && true = {}", true && true);
    println!("true || false = {}", true || false);
    println!("!true = {}", !true);

    println!("0011 AND 0101 = {:04b}", 0b0011 & 0b0101);
    println!("0011 OR 0101 = {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 = {:04b}", 0b0011 ^ 0b0101);

    println!("1 << 5 = {}", 1 << 5);
    println!("0x80 >> 2 = 0x{:X}", 0x80 >> 2);

    println!("one million {}", 1_000_000);
}
