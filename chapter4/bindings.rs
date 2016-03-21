fn main() {
    let an_integer = 99i32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An int: {:?}", copied_integer);

    println!("Bool: {:?}", a_boolean);

    println!("whats a unit? {:?}", unit);

    let _why_do_this = 3u32;

    let mut mutable = 1;

    println!("Val: {}", mutable);

    mutable *= 20;

    println!("Val: {}", mutable);


    let long = 1;
    {
        let short = 2;
        println!("short: {}", short);

        let long = 1_111;
        println!("long: {}", long);
    }

    // println!("short: {}", short);
    println!("long: {}", long);

    let long = 22;

    println!("long: {}", long);


    let test;
    {
        test = 1;
    }

    println!("Test: {}", test);

}
