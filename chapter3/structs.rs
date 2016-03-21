// Unit struct
#[derive(Debug)]
struct Nil;

// a tuple struct
struct Pair(i32, f64);

// struct with two fields
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

// struct with other structs as fields
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    println!("{:?}", Nil);

    let point : Point = Point { x: 1.2 , y: 3.2 };
    println!("{:?}", point);

    let Point { x: my_x, y: my_y } = point;

    let rect : Rectangle = Rectangle {
        p1: Point { x: 0.0, y: 10.0 },
        p2: Point { x: my_x, y: my_y }
    };
    println!("{:?}", rect);

    let pair = Pair(1, 0.111);
    // destrucutre into variables
    let Pair(i, d) = pair;

    println!("pair {:?} and {:?}", i, d);
}
