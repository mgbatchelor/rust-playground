#![allow(dead_code)]

enum Person {
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 }
}

fn inspect(p: Person) {
    match p {
        Person::Skinny    => println!("Skinny"),
        Person::Fat       => println!("Fat"),
        Person::Height(h) => println!("Height is {}", h),
        Person::Weight(w) => println!("Weight is {}", w),
        Person::Info { name, height } => println!("{} is {} tall", name, height)
    }
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

enum Number {
    Zero,
    One,
    Two,
    Three,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    use Status::{Poor, Rich};
    use Work::*;

    let person = Person::Height(18);
    let danny = Person::Weight(10);
    let dave = Person::Info { name: "Dave".to_owned(), height: 72 };
    let john = Person::Fat;
    let larry = Person::Skinny;

    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("the rich."),
        Poor => println!("the poor"),
    }

    match work {
        Civilian => println!("civilians"),
        Soldier => println!("soldiers"),
    }

    println!("zero is {}", Number::Zero as i32);
    println!("two is {}", Number::Two as i32);

    println!("roses are {}", Color::Red as i32);
    println!("violets are {}", Color::Blue as i32);
}
