fn main() {
    let mut n = 0;
    while n < 10 {
        n += 1;
    }
    println!("while: {}", n);

    for i in 0..10 {
        println!("for: {}", i);
    }

    let mut n = 0;
    loop {
        n += 1;
        if n == 2 {
            continue;
        }
        if n == 8 {
            break;
        };

        println!("loop: {}", n);
    }
}
