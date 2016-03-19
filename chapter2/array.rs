use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn last(array: &[i32]) -> i32 {
    array[array.len() - 1]
}

fn sum(array: &[i32]) -> i32 {
    let mut total : i32 = 0;
    for (_, value) in array.iter().enumerate() {
        total = total + value;
    }
    total
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element: {}", xs[0]);
    println!("second element: {}", xs[1]);

    println!("array size: {}", xs.len());

    println!("size of array {} bytes", mem::size_of_val(&xs));

    println!("size of array {} bytes", mem::size_of_val(&ys));

    analyze_slice(&xs);

    analyze_slice(&ys);

    analyze_slice(&ys[1 .. 4]);

    // println!("{}", xs[5]); // index out of bounds

    let last_value = last(&xs);
    println!("Last: {}", last_value);

    println!("Total: {}", sum(&xs));
    println!("Total: {}", sum(&ys));
}
