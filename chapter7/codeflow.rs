fn main() {
    let n = 2;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!("small, so increase");
            10 * n
        } else {
            n / 2
        };

    println!("{} -> {}", n, big_n);

    let mut count = 0u32;

    println!("Loop until the end of the world");
    loop {
        count += 1;

        if count % 3 == 0 {
            println!("Mod 3");
            continue;
        }

        println!("{}", count);

        if count == 10 {
            println!("Done.");
            break;
        }
    }


    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    for n in 1..101 {
        if n % 10 == 0 {
            println!("tens of {}", n);
        } else {
            println!("{}", n);
        }
    }


    let boolean = true;
    let res = match boolean {
        true => 1,
        false => 0,
    };

    println!("Result of {} is {}", boolean, res);
}
