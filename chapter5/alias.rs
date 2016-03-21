type NanoSecond = u64;
type Inch = u64;

fn main() {
    let nanosecond : NanoSecond = 5;

    let inch : Inch = 100;

    println!("{}ns", nanosecond);
    println!("{} inches", inch);

    println!("Combined {}", nanosecond + inch);
}
