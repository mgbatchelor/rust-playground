fn main() {
    println!("Hello World!");

    println!("Var = {}", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{value:>0width$}", value=1, width=10);


    // struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));
    let pi = 22.0/7.0;
    println!("PI: {:.3}", pi);

}
