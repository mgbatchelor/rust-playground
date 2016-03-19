fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 4.1; // f64
    let default_integer = 3; // i32


    let mut mutable = 12; // mutable?

    println!("Mut {}", mutable);
    mutable += 1;
    println!("Mut {}", mutable);

    // default_integer += 1 // fails to compile
}
